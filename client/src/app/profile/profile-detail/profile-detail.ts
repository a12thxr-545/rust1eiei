import { Component, inject, OnInit, signal, computed, effect } from '@angular/core';
import { CommonModule } from '@angular/common';
import { ActivatedRoute, RouterLink } from '@angular/router';
import { MemberService } from '../../_services/member.service';
import { SocialService } from '../../_services/social-service';
import { PassportService } from '../../_services/passport-service';
import { SnackbarService } from '../../_services/snackbar.service';
import { Friend, FriendshipStatus } from '../../_model/social';
import { User } from '../../_model/pagination';

@Component({
    selector: 'app-profile-detail',
    standalone: true,
    imports: [CommonModule, RouterLink],
    templateUrl: './profile-detail.html',
    styleUrl: './profile-detail.css',
})
export class ProfileDetail implements OnInit {
    private _route = inject(ActivatedRoute);
    private _memberService = inject(MemberService);
    private _socialService = inject(SocialService);
    private _passportService = inject(PassportService);
    private _snackbar = inject(SnackbarService);

    user = signal<User | null>(null);
    isRemoving = signal(false);
    isLoading = signal(true);
    friendshipStatus = signal<FriendshipStatus | null>(null);

    isMyProfile = computed(() => this.user()?.id === this._passportService.data()?.id);

    isFriend = computed(() => this.friendshipStatus()?.status.toLowerCase() === 'accepted');

    isRequested = computed(() => {
        const status = this.friendshipStatus();
        const myId = this._passportService.data()?.id;
        return status?.status.toLowerCase() === 'pending' && status?.initiator_id === myId;
    });

    isIncoming = computed(() => {
        const status = this.friendshipStatus();
        const myId = this._passportService.data()?.id;
        return status?.status.toLowerCase() === 'pending' && status?.initiator_id !== myId;
    });

    constructor() {
        effect(() => {
            const u = this.user();
            const p = this._passportService.data();
            if (u && p) {
                this.loadFriendshipStatus(u.id);
            }
        });
    }

    ngOnInit(): void {
        this._route.params.subscribe(params => {
            const username = params['username'];
            if (username) {
                this.loadProfile(username);
            }
        });
        if (this._passportService.data()) {
            this._socialService.loadFriends();
        }
    }

    loadProfile(username: string) {
        this.isLoading.set(true);
        this.friendshipStatus.set(null);
        this._memberService.getProfileByUsername(username).subscribe({
            next: (user) => {
                this.user.set(user);
                this.isLoading.set(false);
            },
            error: () => {
                this._snackbar.error('User not found');
                this.isLoading.set(false);
            }
        });
    }

    async loadFriendshipStatus(otherId: number) {
        if (!this._passportService.data()) {
            this.friendshipStatus.set({ status: 'none', friendship_id: null, initiator_id: null });
            return;
        }
        try {
            const status = await this._socialService.getFriendshipStatus(otherId);
            this.friendshipStatus.set(status);
        } catch (e) {
            console.error('Failed to load friendship status', e);
            this._snackbar.error('Could not verify friendship status');
            this.friendshipStatus.set({ status: 'none', friendship_id: null, initiator_id: null });
        }
    }

    async addFriend() {
        const user = this.user();
        if (!user) return;
        try {
            await this._socialService.addFriend(user.id);
            this._snackbar.success('Friend request sent!');
            await this.loadFriendshipStatus(user.id);
        } catch (e: any) {
            this._snackbar.error(e.error || 'Failed to send friend request');
        }
    }

    async acceptFriend() {
        const user = this.user();
        if (!user) return;
        try {
            await this._socialService.acceptFriend(user.id);
            this._snackbar.success('Friend request accepted!');
            await this.loadFriendshipStatus(user.id);
        } catch (e: any) {
            this._snackbar.error('Failed to accept friend request');
        }
    }

    async removeFriend() {
        const user = this.user();
        if (!user) return;
        if (!confirm(`Are you sure you want to remove ${user.display_name} from your friends?`)) return;

        this.isRemoving.set(true);
        try {
            await this._socialService.removeFriend(user.id);
            this._snackbar.success('Friend removed');
            await this.loadFriendshipStatus(user.id);
        } catch (e) {
            this._snackbar.error('Failed to remove friend');
        } finally {
            this.isRemoving.set(false);
        }
    }
}
