import { Component, inject, OnInit, signal, computed, effect, PLATFORM_ID, OnDestroy, ChangeDetectorRef } from '@angular/core';
import { isPlatformBrowser } from '@angular/common';
import { MissionService } from '../_services/mission-service';
import { PassportService } from '../_services/passport-service';
import { SnackbarService } from '../_services/snackbar.service';
import { DatePipe } from '@angular/common';
import { CrewMember, Mission } from '../_model/mission';
import { FormsModule } from '@angular/forms';
import { RouterLink, Router, NavigationEnd, ActivatedRoute } from '@angular/router';
import { filter } from 'rxjs';
import { SocialService } from '../_services/social-service';
import { MemberService } from '../_services/member.service';
import { MissionInvitation } from '../_model/social';
import { Subscription } from 'rxjs';

@Component({
  selector: 'app-mission',
  imports: [DatePipe, FormsModule, RouterLink],
  templateUrl: './mission.html',
  styleUrl: './mission.css',
})
export class MissionComponent implements OnInit, OnDestroy {
  private _missionService = inject(MissionService);
  public passportService = inject(PassportService);
  private _snackbar = inject(SnackbarService);
  public socialService = inject(SocialService);
  private _memberService = inject(MemberService);
  private _platformId = inject(PLATFORM_ID);
  private _router = inject(Router);
  private _activatedRoute = inject(ActivatedRoute);
  private _cdr = inject(ChangeDetectorRef);
  private _subscriptions = new Subscription();

  constructor() {
    if (isPlatformBrowser(this._platformId)) {
      this._timeTicker = setInterval(() => {
        this.now.set(Date.now());
        this._cdr.detectChanges();
      }, 1000);
    }

    effect(() => {
      if (this.passportService.data()) {
        // Wrap in setTimeout to push to next macrotask and avoid NG0100
        setTimeout(() => {
          this.refreshAll();
          this._cdr.detectChanges();
        });
      }
    });

    if (isPlatformBrowser(this._platformId)) {
      this._router.events.pipe(
        filter(event => event instanceof NavigationEnd)
      ).subscribe(() => {
        setTimeout(() => {
          this.refreshAll();
        });
      });

      this._subscriptions.add(
        this._missionService.refresh$.subscribe(() => {
          setTimeout(() => {
            this.refreshAll();
          });
        })
      );
    }
  }

  ngOnInit(): void {
    this._activatedRoute.queryParams.subscribe(params => {
      const missionId = params['view'];
      if (missionId) {
        this.openMissionById(Number(missionId));
      }
    });
  }

  ngOnDestroy(): void {
    if (this._timeTicker) clearInterval(this._timeTicker);
    if (this.pollInterval) clearInterval(this.pollInterval);
    this._subscriptions.unsubscribe();
  }

  private async openMissionById(id: number) {
    this.isProcessing.set(true);
    try {
      const mission = await this._missionService.getMission(id);
      if (mission) {
        this.openViewModal(mission);
      }
    } finally {
      this.isProcessing.set(false);
    }
  }

  private refreshAll() {
    const passport = this.passportService.data();
    if (!passport) return;

    this.loadOtherMissions();
    this.loadMyMissions();
    this.loadJoinedMissions();
    this.loadFinishedMissions();
    this._missionService.getCurrentMission();
    this.loadSocialData();
  }

  missions = this._missionService.missions;
  myMissions = this._missionService.myMissions;
  joinedMissions = this._missionService.joinedMissions;
  finishedMissions = this._missionService.finishedMissions;
  isLoading = this._missionService.isLoading;
  isLoadingMyMissions = this._missionService.isLoadingMyMissions;
  isLoadingFinishedMissions = this._missionService.isLoadingFinishedMissions;
  currentMissionId = this._missionService.currentMissionId;
  selectedTabIndex = 0;

  // Modal states
  showViewModal = signal(false);
  showEditModal = signal(false);
  showDeleteModal = signal(false);
  selectedMission = signal<Mission | null>(null);
  isProcessing = signal(false);
  crewMembers = signal<CrewMember[]>([]);
  missionInvitations = signal<MissionInvitation[]>([]);
  loadingCrew = signal(false);

  // Invite cooldown tracking
  private inviteCooldowns = new Map<number, number>(); // brawlerId -> timestamp
  public now = signal(Date.now());
  public friendsWithCooldown = computed(() => {
    const now = this.now();
    return this.socialService.friends().map(f => ({
      ...f,
      isCooldown: now - (this.inviteCooldowns.get(f.friend_id) || 0) < 3000
    }));
  });
  private _timeTicker?: any;

  // Polling
  private pollInterval?: any;

  // Search
  searchCode = '';
  brawlerSearchQuery = '';
  brawlerSearchResults = this._memberService.paginator;

  // Edit form
  editName = '';
  editDescription = '';
  editMaxParticipants = 0;

  loadSocialData(): void {
    this.socialService.loadFriends();
    this.socialService.loadPendingRequests();
    this.socialService.loadInvitations();
  }

  loadOtherMissions(): void {
    const passport = this.passportService.data();
    if (passport) {
      this._missionService.loadOtherMissions({
        exclude_chief_id: passport.id,
        exclude_member_id: passport.id,
        code: this.searchCode || undefined
      });
    }
  }

  loadJoinedMissions(): void {
    const passport = this.passportService.data();
    if (passport) {
      this._missionService.loadJoinedMissions(passport.id);
    }
  }

  loadMyMissions(): void {
    const passport = this.passportService.data();
    if (passport) {
      this._missionService.loadMyMissions(passport.id);
    }
  }

  loadFinishedMissions(): void {
    const passport = this.passportService.data();
    if (passport) {
      this._missionService.loadFinishedMissions(passport.id);
    }
  }

  onTabChange(index: number): void {
    this.selectedTabIndex = index;
    if (index === 0) {
      this.loadOtherMissions();
    } else if (index === 1) {
      this.loadJoinedMissions();
    } else if (index === 2) {
      this.loadMyMissions();
    } else if (index === 3) {
      this.loadFinishedMissions();
    }
  }

  getStatusClass(status: string): string {
    return `status-${status.toLowerCase()}`;
  }

  async openViewModal(mission: Mission): Promise<void> {
    this.selectedMission.set(mission);
    this.showViewModal.set(true);
    this.crewMembers.set([]);
    this.missionInvitations.set([]);
    this.loadingCrew.set(true);
    this.loadCrewMembers(mission.id);

    // Start polling when modal opens
    if (isPlatformBrowser(this._platformId)) {
      this.startPolling(mission.id);
    }
  }

  private startPolling(missionId: number) {
    if (this.pollInterval) clearInterval(this.pollInterval);
    this.pollInterval = setInterval(() => {
      this.loadCrewMembers(missionId);
      this.refreshMissionData(missionId);
    }, 5000); // Poll every 5 seconds
  }

  private async refreshMissionData(missionId: number) {
    const updated = await this._missionService.getMission(missionId);
    if (updated) {
      if (this.selectedMission()?.id === missionId) {
        this.selectedMission.set(updated);
      }
    } else {
      // Mission was likely deleted or user was kicked
      if (this.showViewModal() && this.selectedMission()?.id === missionId) {
        this._snackbar.info('This mission is no longer available.');
        this.closeViewModal();
        this.refreshAll();
      }
    }
  }

  async loadCrewMembers(missionId: number): Promise<void> {
    try {
      const [crew, invitations] = await Promise.all([
        this._missionService.getCrewMembers(missionId),
        this.socialService.loadMissionInvitations(missionId)
      ]);
      this.crewMembers.set(crew);
      this.missionInvitations.set(invitations);
    } finally {
      this.loadingCrew.set(false);
    }
  }

  closeViewModal(): void {
    if (this.pollInterval) {
      clearInterval(this.pollInterval);
      this.pollInterval = undefined;
    }
    this.showViewModal.set(false);
    this.selectedMission.set(null);
    this.crewMembers.set([]);
  }

  isFriend(brawlerId: number): boolean {
    return this.socialService.friends().some(f => f.friend_id === brawlerId);
  }

  isMember(brawlerId: number): boolean {
    return this.crewMembers().some(m => m.brawler_id === brawlerId);
  }

  isInvited(brawlerId: number): boolean {
    return this.missionInvitations().some(i => i.invitee_id === brawlerId);
  }

  async viewCurrentMission(): Promise<void> {
    const id = this.currentMissionId();
    if (id === null) return;

    this.isProcessing.set(true);
    try {
      const mission = await this._missionService.getMission(id);
      if (mission) {
        this.openViewModal(mission);
      } else {
        this._snackbar.error('Could not load current mission details');
      }
    } finally {
      this.isProcessing.set(false);
    }
  }

  async addFriend(friendId: number) {
    try {
      await this.socialService.addFriend(friendId);
      this._snackbar.success('Friend request sent!');
    } catch (e: any) {
      this._snackbar.error(e.error || 'Failed to send friend request');
    }
  }

  async inviteFriend(friendId: number): Promise<void> {
    const mission = this.selectedMission();
    if (!mission) return;

    const lastInvite = this.inviteCooldowns.get(friendId) || 0;
    const now = Date.now();
    if (now - lastInvite < 3000) {
      const remaining = Math.ceil((3000 - (now - lastInvite)) / 1000);
      this._snackbar.warning(`Please wait ${remaining}s before re-inviting.`);
      return;
    }

    try {
      await this.socialService.inviteToMission(friendId, mission.id);
      this._snackbar.success('Invitation sent!');
      this.inviteCooldowns.set(friendId, now);
      this.loadCrewMembers(mission.id);
    } catch (e: any) {
      this._snackbar.error(e.error || 'Failed to send invitation');
    }
  }

  isInvitationInCooldown(friendId: number): boolean {
    const lastInvite = this.inviteCooldowns.get(friendId) || 0;
    return (this.now() - lastInvite < 3000);
  }

  async respondToInvite(invitationId: number, accept: boolean): Promise<void> {
    try {
      const mid = await this.socialService.respondToInvitation(invitationId, accept);
      if (accept) {
        this._snackbar.success('Joined mission!');
        this.refreshAll();
        this.openMissionById(mid);
      } else {
        this._snackbar.success('Invitation rejected');
      }
    } catch (e: any) {
      const errorMsg = e.error?.error || e.error || 'Failed to respond to invitation';
      this._snackbar.error(errorMsg);
    }
  }

  async acceptFriend(friendId: number): Promise<void> {
    try {
      await this.socialService.acceptFriend(friendId);
      this._snackbar.success('Friend accepted!');
    } catch (e: any) {
      this._snackbar.error(e.error || 'Failed to accept friend');
    }
  }

  async rejectFriend(friendId: number): Promise<void> {
    try {
      await this.socialService.rejectFriend(friendId);
      this._snackbar.success('Request rejected');
    } catch (e: any) {
      this._snackbar.error(e.error || 'Failed to reject friend');
    }
  }

  openEditModal(mission: Mission): void {
    this.selectedMission.set(mission);
    this.editName = mission.name;
    this.editDescription = mission.description || '';
    this.editMaxParticipants = mission.max_participants;
    this.showEditModal.set(true);
  }

  closeEditModal(): void {
    this.showEditModal.set(false);
    this.selectedMission.set(null);
  }

  async submitEdit(): Promise<void> {
    const mission = this.selectedMission();
    if (!mission) return;

    this.isProcessing.set(true);
    const error = await this._missionService.editMission(mission.id, {
      name: this.editName || undefined,
      description: this.editDescription || undefined,
      max_participants: this.editMaxParticipants
    });
    this.isProcessing.set(false);

    if (error) {
      this._snackbar.error(error);
    } else {
      this._snackbar.success('Mission updated successfully');
      this.closeEditModal();
      this.loadMyMissions();
    }
  }

  openDeleteModal(mission: Mission): void {
    this.selectedMission.set(mission);
    this.showDeleteModal.set(true);
  }

  closeDeleteModal(): void {
    this.showDeleteModal.set(false);
    this.selectedMission.set(null);
  }

  async confirmDelete(): Promise<void> {
    const mission = this.selectedMission();
    if (!mission) return;

    this.isProcessing.set(true);
    const error = await this._missionService.deleteMission(mission.id);
    this.isProcessing.set(false);

    if (error) {
      this._snackbar.error(error);
    } else {
      this._snackbar.success('Mission deleted successfully');
      this.closeDeleteModal();
      this.loadMyMissions();
      this._missionService.getCurrentMission();
    }
  }

  async joinMission(mission: Mission): Promise<void> {

    this.isProcessing.set(true);
    const error = await this._missionService.joinMission(mission.id);
    this.isProcessing.set(false);

    if (error) {
      this._snackbar.error(error);
    } else {
      this._snackbar.success(`Successfully joined "${mission.name}"`);
      this.loadOtherMissions();
      this._missionService.getCurrentMission();
    }
  }

  async leaveMission(mission: Mission): Promise<void> {
    this.isProcessing.set(true);
    const error = await this._missionService.leaveMission(mission.id);
    this.isProcessing.set(false);

    if (error) {
      this._snackbar.error(error);
    } else {
      this._snackbar.success(`Left "${mission.name}"`);
      this.refreshAll();
    }
  }

  async kickMember(brawlerId: number): Promise<void> {
    const mission = this.selectedMission();
    if (!mission) return;

    this.isProcessing.set(true);
    const error = await this._missionService.kickMember(mission.id, brawlerId);
    this.isProcessing.set(false);

    if (error) {
      this._snackbar.error(error);
    } else {
      this._snackbar.success('Member removed from mission');
      this.loadCrewMembers(mission.id);
    }
  }

  isInMission(missionId: number): boolean {
    return this.currentMissionId() === missionId;
  }

  isFull(mission: Mission): boolean {
    return mission.max_participants > 0 && mission.crew_count >= mission.max_participants;
  }

  isOverLimit(mission: Mission): boolean {
    return mission.max_participants > 0 && mission.crew_count > mission.max_participants;
  }

  async startMission(missionId: number): Promise<void> {
    this.isProcessing.set(true);
    const error = await this._missionService.startMission(missionId);
    this.isProcessing.set(false);

    if (error) {
      this._snackbar.error(error);
    } else {
      this._snackbar.success('Mission started!');
      this.refreshMissionDetails(missionId);
    }
  }

  async completeMission(missionId: number): Promise<void> {
    this.isProcessing.set(true);
    const error = await this._missionService.completeMission(missionId);
    this.isProcessing.set(false);

    if (error) {
      this._snackbar.error(error);
    } else {
      this._snackbar.success('Mission success! Congratulations!');
      this._missionService.getCurrentMission();
      this.refreshMissionDetails(missionId);
    }
  }

  async failMission(missionId: number): Promise<void> {
    this.isProcessing.set(true);
    const error = await this._missionService.failMission(missionId);
    this.isProcessing.set(false);

    if (error) {
      this._snackbar.error(error);
    } else {
      this._snackbar.warning('Mission ended (Failed).');
      this._missionService.getCurrentMission();
      this.refreshMissionDetails(missionId);
    }
  }

  isLeader(mission: Mission | null): boolean {
    if (!mission) return false;
    return mission.chief_id === this.passportService.data()?.id;
  }

  private async refreshMissionDetails(missionId: number): Promise<void> {
    this.refreshAll();
    const selected = this.selectedMission();
    if (selected && selected.id === missionId) {
      const updatedMission = await this._missionService.getMission(missionId);
      if (updatedMission) {
        this.selectedMission.set(updatedMission);
      }
    }
  }

  onBrawlerSearch() {
    const paginator = this.brawlerSearchResults();
    paginator.pagination.query = this.brawlerSearchQuery;
    paginator.pagination.currentPage = 1;
    this._memberService.paginator.set(paginator);
    this._memberService.getMember();
  }
}
