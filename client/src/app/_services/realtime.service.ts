import { Injectable, inject, PLATFORM_ID, NgZone, effect } from '@angular/core';
import { isPlatformBrowser } from '@angular/common';
import { environment } from '../../environments/environment.development';
import { PassportService } from './passport-service';
import { SocialService } from './social-service';
import { SnackbarService } from './snackbar.service';

@Injectable({
    providedIn: 'root'
})
export class RealtimeService {
    private _passportService = inject(PassportService);
    private _socialService = inject(SocialService);
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
        } else if (event.type === 'MissionInvitation') {
            this._snackbar.info(`Mission Invite: ${event.payload.mission_id}`);
            this._socialService.loadInvitations();
        }
    }
}
