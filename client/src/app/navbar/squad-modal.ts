import { Component, inject, signal, Output, EventEmitter, OnInit } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { RouterLink } from '@angular/router';
import { SocialService } from '../_services/social-service';
import { PassportService } from '../_services/passport-service';
import { MemberService } from '../_services/member.service';
import { SnackbarService } from '../_services/snackbar.service';

@Component({
    selector: 'app-squad-modal',
    standalone: true,
    imports: [CommonModule, FormsModule, RouterLink],
    templateUrl: './squad-modal.html',
    styleUrl: './squad-modal.css'
})
export class SquadModal implements OnInit {
    public socialService = inject(SocialService);
    public passportService = inject(PassportService);
    private _memberService = inject(MemberService);
    private _snackbar = inject(SnackbarService);

    @Output() close = new EventEmitter<void>();

    brawlerSearchQuery = '';
    brawlerSearchResults = this._memberService.paginator;

    constructor() {
        this.socialService.loadFriends();
    }

    ngOnInit(): void {
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
}
