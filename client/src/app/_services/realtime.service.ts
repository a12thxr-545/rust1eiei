import { Injectable, inject, PLATFORM_ID, NgZone, effect } from '@angular/core';
import { isPlatformBrowser } from '@angular/common';
import { environment } from '../../environments/environment.development';
import { PassportService } from './passport-service';
import { SocialService } from './social-service';
import { MissionService } from './mission-service';
import { SnackbarService } from './snackbar.service';

@Injectable({
    providedIn: 'root'
})
export class RealtimeService {
    private _passportService = inject(PassportService);
    private _socialService = inject(SocialService);
    private _missionService = inject(MissionService);
    private _snackbar = inject(SnackbarService);
    private _platformId = inject(PLATFORM_ID);
    private _zone = inject(NgZone);

    private _eventSource: any = null;

    constructor() {
        if (isPlatformBrowser(this._platformId)) {
            effect(() => {
                const data = this._passportService.data();
                if (data?.access_token) {
                    this.connect(data.access_token);
                } else {
                    this.disconnect();
                }
            });
        }
    }

    private connect(token: string) {
        this.disconnect();

        // Pass token as query param for SSE
        const url = `${environment.base_url}/api/social/events?token=${token}`;
        this._eventSource = new (window as any).EventSource(url);

        this._eventSource.onmessage = (event: any) => {
            this._zone.run(() => {
                try {
                    const data = JSON.parse(event.data);
                    this.handleEvent(data);
                } catch (e) {
                    console.error('Failed to parse realtime event:', e);
                }
            });
        };

        this._eventSource.onerror = (error: any) => {
            console.error('SSE Error:', error);
            // Browser will usually handle automatic reconnection for SSE
        };
    }

    private disconnect() {
        if (this._eventSource) {
            this._eventSource.close();
            this._eventSource = null;
        }
    }

    private handleEvent(event: any) {
        console.log('Realtime Event received:', event);
        if (event.type === 'FriendRequest') {
            this._snackbar.info('New friend request received!');
            this._socialService.loadPendingRequests();
        } else if (event.type === 'FriendAccepted') {
            this._snackbar.success('Friend request accepted!');
            this._socialService.loadFriends();
        } else if (event.type === 'MissionInvitation') {
            this._snackbar.info('You have been invited to a mission!');
            this._socialService.loadInvitations();
        } else if (event.type === 'MissionCreated' || event.type === 'MissionUpdated' ||
            event.type === 'MissionDeleted' || event.type === 'MissionStatusChanged' ||
            event.type === 'MissionJoined' ||
            event.type === 'MissionInvitationAccepted') {
            if (event.type === 'MissionDeleted') {
                this._snackbar.info('The mission has been terminated by the chief.');
            }
            this._missionService.triggerRefresh();
            this.refreshMissions();
        } else if (event.type === 'MissionLeft') {
            const passport = this._passportService.data();
            if (passport && event.payload.brawler_id === passport.id) {
                this._snackbar.warning('System Alert: You have been removed from the mission deployment.');
            }
            this._missionService.triggerRefresh();
            this.refreshMissions();
        }
    }

    private refreshMissions() {
        const passport = this._passportService.data();
        if (!passport) return;

        // Refresh all relevant lists
        this._missionService.loadOtherMissions({
            exclude_chief_id: passport.id,
            exclude_member_id: passport.id
        });
        this._missionService.loadMyMissions(passport.id);
        this._missionService.loadJoinedMissions(passport.id);
        this._missionService.loadFinishedMissions(passport.id);
        this._missionService.getCurrentMission();

        // Also refresh social data to update friends' mission status
        this._socialService.loadFriends();
        this._socialService.loadPendingRequests();
        this._socialService.loadInvitations();
    }
}
