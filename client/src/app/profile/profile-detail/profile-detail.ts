import { Component, inject, OnInit, signal, computed } from '@angular/core';
import { CommonModule } from '@angular/common';
import { ActivatedRoute, RouterLink } from '@angular/router';
import { MemberService } from '../../_services/member.service';
import { SocialService } from '../../_services/social-service';
import { PassportService } from '../../_services/passport-service';
import { SnackbarService } from '../../_services/snackbar.service';
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
    isLoading = signal(true);
    isMyProfile = computed(() => this.user()?.id === this._passportService.data()?.id);
    isFriend = computed(() => {
        const userId = this.user()?.id;
        if (!userId) return false;
        return this._socialService.friends().some(f => f.friend_id === userId);
    });

    ngOnInit(): void {
        this._route.params.subscribe(params => {
            const username = params['username'];
            if (username) {
                this.loadProfile(username);
            }
        });
        this._socialService.loadFriends();
    }

    loadProfile(username: string) {
        this.isLoading.set(true);
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

    async addFriend() {
        const user = this.user();
        if (!user) return;
        try {
            await this._socialService.addFriend(user.id);
            this._snackbar.success('Friend request sent!');
        } catch (e: any) {
            this._snackbar.error(e.error || 'Failed to send friend request');
        }
    }
}
