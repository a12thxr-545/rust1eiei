import { Component, inject, OnInit, signal, effect, PLATFORM_ID } from '@angular/core';
import { isPlatformBrowser } from '@angular/common';
import { MissionService } from '../_services/mission-service';
import { PassportService } from '../_services/passport-service';
import { SnackbarService } from '../_services/snackbar.service';
import { DatePipe } from '@angular/common';
import { CrewMember, Mission } from '../_model/mission';
import { FormsModule } from '@angular/forms';
import { RouterLink, Router, NavigationEnd, ActivatedRoute } from '@angular/router';
import { filter, firstValueFrom } from 'rxjs';
import { SocialService } from '../_services/social-service';
import { MemberService } from '../_services/member.service';
import { RatingService } from '../_services/rating.service';
import { MissionRatingSummary } from '../_model/rating';

@Component({
  selector: 'app-mission',
  imports: [DatePipe, FormsModule, RouterLink],
  templateUrl: './mission.html',
  styleUrl: './mission.css',
})
export class MissionComponent implements OnInit {
  private _missionService = inject(MissionService);
  public passportService = inject(PassportService);
  private _snackbar = inject(SnackbarService);
  public socialService = inject(SocialService);
  private _memberService = inject(MemberService);
  private _platformId = inject(PLATFORM_ID);
  private _router = inject(Router);
  private _ratingService = inject(RatingService);
  private _activatedRoute = inject(ActivatedRoute);

  constructor() {
    // Basic effect for when passport changes
    effect(() => {
      if (this.passportService.data()) {
        this.refreshAll();
      }
    });

    // Handle navigation refresh
    if (isPlatformBrowser(this._platformId)) {
      this._router.events.pipe(
        filter(event => event instanceof NavigationEnd)
      ).subscribe(() => {
        this.refreshAll();
      });

      // Handle tab switching via query params
      this._activatedRoute.queryParams.subscribe(params => {
        if (params['tab'] === 'squad') {
          this.onTabChange(3);
        }
      });
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
  showInviteModal = signal(false);
  selectedMission = signal<Mission | null>(null);
  isProcessing = signal(false);
  crewMembers = signal<CrewMember[]>([]);
  loadingCrew = signal(false);

  // Search
  searchCode = '';
  brawlerSearchQuery = '';
  brawlerSearchResults = this._memberService.paginator;

  // Rating states
  missionRatings = signal<MissionRatingSummary | null>(null);
  myRating = signal<number | null>(null);
  selectedRating = signal<number>(0);
  ratingComment = signal<string>('');
  isSubmittingRating = signal(false);

  isFriend(brawlerId: number): boolean {
    return this.socialService.friends().some(f => f.friend_id === brawlerId);
  }

  onBrawlerSearch() {
    const paginator = this.brawlerSearchResults();
    paginator.pagination.query = this.brawlerSearchQuery;
    paginator.pagination.currentPage = 1;
    this._memberService.paginator.set(paginator);
    this._memberService.getMember();
  }

  async addFriend(friendId: number) {
    try {
      await this.socialService.addFriend(friendId);
      this._snackbar.success('Friend request sent!');
    } catch (e: any) {
      this._snackbar.error(e.error || 'Failed to send friend request');
    }
  }

  // Edit form
  editName = '';
  editDescription = '';

  ngOnInit(): void {
    if (isPlatformBrowser(this._platformId)) {
      // Mission data is loaded via the effect when passport data is available
    }
  }

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

  private loadJoinedMissions(): void {
    const passport = this.passportService.data();
    if (passport) {
      this._missionService.loadJoinedMissions(passport.id);
    }
  }

  private loadMyMissions(): void {
    const passport = this.passportService.data();
    if (passport) {
      this._missionService.loadMyMissions(passport.id);
    }
  }

  private loadFinishedMissions(): void {
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
      this.loadSocialData();
    } else if (index === 4) {
      this.loadFinishedMissions();
    }
  }

  getStatusClass(status: string): string {
    return `status-${status.toLowerCase()}`;
  }

  // View Mission
  async openViewModal(mission: Mission): Promise<void> {
    this.selectedMission.set(mission);
    this.showViewModal.set(true);
    this.crewMembers.set([]);
    this.loadingCrew.set(true);
    this.missionRatings.set(null);
    this.myRating.set(null);
    this.selectedRating.set(0);
    this.ratingComment.set('');

    this.loadCrewMembers(mission.id);
    this.loadRatings(mission.id);
  }

  async loadCrewMembers(missionId: number): Promise<void> {
    try {
      const crew = await this._missionService.getCrewMembers(missionId);
      this.crewMembers.set(crew);
    } finally {
      this.loadingCrew.set(false);
    }
  }

  openImagePreview(url: string | undefined): void {
    if (url) {
      window.open(url, '_blank');
    }
  }

  // Rating methods
  async loadRatings(missionId: number): Promise<void> {
    const ratings = await this._ratingService.getMissionRatings(missionId);
    this.missionRatings.set(ratings);

    const myRating = await this._ratingService.getMyRating(missionId);
    this.myRating.set(myRating);
    if (myRating) {
      this.selectedRating.set(myRating);
    }
  }

  setRating(rating: number): void {
    // Don't allow changing if already rated
    if (this.myRating() !== null) return;
    this.selectedRating.set(rating);
  }

  async submitRating(): Promise<void> {
    const mission = this.selectedMission();
    const rating = this.selectedRating();
    if (!mission || rating === 0) return;

    this.isSubmittingRating.set(true);
    const result = await this._ratingService.addRating(
      mission.id,
      rating,
      this.ratingComment() || undefined
    );
    this.isSubmittingRating.set(false);

    if (typeof result === 'string') {
      this._snackbar.error(result);
    } else {
      this._snackbar.success('Rating submitted!');
      this.loadRatings(mission.id);
    }
  }

  closeViewModal(): void {
    this.showViewModal.set(false);
    this.selectedMission.set(null);
    this.crewMembers.set([]);
  }

  // Invitations
  async inviteFriend(friendId: number): Promise<void> {
    const mission = this.selectedMission();
    if (!mission) return;

    try {
      await this.socialService.inviteToMission(friendId, mission.id);
      this._snackbar.success('Invitation sent!');
    } catch (e: any) {
      this._snackbar.error(e.error || 'Failed to send invitation');
    }
  }

  async respondToInvite(invitationId: number, accept: boolean): Promise<void> {
    try {
      await this.socialService.respondToInvitation(invitationId, accept);
      if (accept) {
        this._snackbar.success('Joined mission!');
        this.loadOtherMissions();
        this.loadMyMissions();
        this._missionService.getCurrentMission();
      } else {
        this._snackbar.success('Invitation rejected');
      }
    } catch (e: any) {
      this._snackbar.error(e.error || 'Failed to respond to invitation');
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

  // Edit Mission
  openEditModal(mission: Mission): void {
    this.selectedMission.set(mission);
    this.editName = mission.name;
    this.editDescription = mission.description || '';
    this.showEditModal.set(true);
  }

  closeEditModal(): void {
    this.showEditModal.set(false);
    this.selectedMission.set(null);
    this.editName = '';
    this.editDescription = '';
  }

  async submitEdit(): Promise<void> {
    const mission = this.selectedMission();
    if (!mission) return;

    this.isProcessing.set(true);
    const error = await this._missionService.editMission(mission.id, {
      name: this.editName || undefined,
      description: this.editDescription || undefined,
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

  // Delete Mission
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

  // Join Mission
  async joinMission(mission: Mission): Promise<void> {
    const currentMissionId = this.currentMissionId();
    if (currentMissionId !== null) {
      this._snackbar.warning('You are already in a mission. Leave it first before joining another.');
      return;
    }

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

  // Leave Mission
  async leaveMission(mission: Mission): Promise<void> {
    this.isProcessing.set(true);
    const error = await this._missionService.leaveMission(mission.id);
    this.isProcessing.set(false);

    if (error) {
      this._snackbar.error(error);
    } else {
      this._snackbar.success(`Left "${mission.name}"`);
      this.loadOtherMissions();
      this.loadMyMissions();
      this._missionService.getCurrentMission();
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
      // Refresh crew list
      const crew = await this._missionService.getCrewMembers(mission.id);
      this.crewMembers.set(crew);
    }
  }

  isInMission(missionId: number): boolean {
    return this.currentMissionId() === missionId;
  }

  async startMission(missionId: number): Promise<void> {
    if (!this.isLeader(this.selectedMission())) {
      this._snackbar.error('Only the mission leader can start the mission');
      return;
    }

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
    if (!this.isLeader(this.selectedMission())) {
      this._snackbar.error('Only the mission leader can complete the mission');
      return;
    }

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
    if (!this.isLeader(this.selectedMission())) {
      this._snackbar.error('Only the mission leader can fail the mission');
      return;
    }

    this.isProcessing.set(true);
    const error = await this._missionService.failMission(missionId);
    this.isProcessing.set(false);

    if (error) {
      this._snackbar.error(error);
    } else {
      this._snackbar.error('Mission failed. Try again!');
      this.refreshMissionDetails(missionId);
    }
  }

  isLeader(mission: Mission | null): boolean {
    if (!mission) return false;
    return mission.chief_id === this.passportService.data()?.id;
  }

  private async refreshMissionDetails(missionId: number): Promise<void> {
    // Reload missions to update list
    this.loadOtherMissions();
    this.loadJoinedMissions();
    this.loadMyMissions();

    // Update selected mission if it's the one being viewed
    const selected = this.selectedMission();
    if (selected && selected.id === missionId) {
      const updatedMission = await this._missionService.getMission(missionId);
      if (updatedMission) {
        this.selectedMission.set(updatedMission);
      }
    }
  }
}
