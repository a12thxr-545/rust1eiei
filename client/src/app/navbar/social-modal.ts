import { Component, inject, signal, ElementRef, ViewChild, Output, EventEmitter } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { RouterLink } from '@angular/router';
import { SocialService } from '../_services/social-service';
import { PassportService } from '../_services/passport-service';
import { MemberService } from '../_services/member.service';
import { SnackbarService } from '../_services/snackbar.service';

@Component({
    selector: 'app-social-modal',
    standalone: true,
    imports: [CommonModule, FormsModule, RouterLink],
    templateUrl: './social-modal.html',
    styleUrl: './social-modal.css'
})
export class SocialModal {
    public socialService = inject(SocialService);
    public passportService = inject(PassportService);
    private _memberService = inject(MemberService);
    private _snackbar = inject(SnackbarService);

    @Output() close = new EventEmitter<void>();

    brawlerSearchQuery = '';
    brawlerSearchResults = this._memberService.paginator;

    constructor() {
        this.socialService.loadFriends();
        this.socialService.loadPendingRequests();
        this.socialService.loadInvitations();
    }

    onClose() {
        this.close.emit();
    }

    onBrawlerSearch() {
        const paginator = this.brawlerSearchResults();
        paginator.pagination.query = this.brawlerSearchQuery;
        paginator.pagination.currentPage = 1;
        this._memberService.paginator.set(paginator);
        this._memberService.getMember();
    }

    isFriend(brawlerId: number): boolean {
        return this.socialService.friends().some(f => f.friend_id === brawlerId);
    }

    async addFriend(friendId: number) {
        try {
            await this.socialService.addFriend(friendId);
            this._snackbar.success('Friend request sent!');
        } catch (e: any) {
            this._snackbar.error(e.error || 'Failed to send friend request');
        }
    }

    async acceptFriend(friendId: number) {
        try {
            await this.socialService.acceptFriend(friendId);
            this._snackbar.success('Friend accepted!');
        } catch (e: any) {
            this._snackbar.error(e.error || 'Failed to accept friend');
        }
    }

    async rejectFriend(friendId: number) {
        try {
            await this.socialService.rejectFriend(friendId);
            this._snackbar.success('Request rejected');
        } catch (e: any) {
            this._snackbar.error(e.error || 'Failed to reject friend');
        }
    }

    async respondToInvite(invitationId: number, accept: boolean) {
        try {
            await this.socialService.respondToInvitation(invitationId, accept);
            if (accept) {
                this._snackbar.success('Joined mission!');
            } else {
                this._snackbar.success('Invitation rejected');
            }
        } catch (e: any) {
            this._snackbar.error(e.error || 'Failed to respond to invitation');
        }
    }
}
