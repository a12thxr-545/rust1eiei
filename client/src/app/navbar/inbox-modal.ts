import { Component, inject, Output, EventEmitter, OnInit } from '@angular/core';
import { CommonModule } from '@angular/common';
import { Router, RouterLink } from '@angular/router';
import { SocialService } from '../_services/social-service';
import { PassportService } from '../_services/passport-service';
import { MissionService } from '../_services/mission-service';
import { SnackbarService } from '../_services/snackbar.service';

@Component({
    selector: 'app-inbox-modal',
    standalone: true,
    imports: [CommonModule, RouterLink],
    templateUrl: './inbox-modal.html',
    styleUrl: './inbox-modal.css'
})
export class InboxModal implements OnInit {
    public socialService = inject(SocialService);
    public passportService = inject(PassportService);
    private _snackbar = inject(SnackbarService);
    private _router = inject(Router);
    private _missionService = inject(MissionService);

    @Output() close = new EventEmitter<void>();

    ngOnInit(): void {
        this.socialService.loadPendingRequests();
        this.socialService.loadInvitations();
    }

    onClose() {
        this.close.emit();
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
            const missionId = await this.socialService.respondToInvitation(invitationId, accept);
            if (accept) {
                this._snackbar.success('Joined mission!');
                await this._missionService.getCurrentMission();

                // Close modal and navigate to mission view
                this.onClose();
                this._router.navigate(['/missions'], { queryParams: { view: missionId } });
            } else {
                this._snackbar.success('Invitation rejected');
            }
        } catch (e: any) {
            const errorMsg = e.error?.error || e.error || 'Failed to respond to invitation';
            this._snackbar.error(errorMsg);
        }
    }

    viewMission(missionId: number) {
        this.onClose();
        this._router.navigate(['/missions'], { queryParams: { view: missionId } });
    }
}
