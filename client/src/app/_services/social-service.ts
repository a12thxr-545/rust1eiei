import { HttpClient } from '@angular/common/http';
import { Injectable, inject, signal, PLATFORM_ID } from '@angular/core';
import { isPlatformBrowser } from '@angular/common';
import { environment } from '../../environments/environment.development';
import { Friend, FriendshipStatus, MissionInvitation } from '../_model/social';
import { lastValueFrom } from 'rxjs';

@Injectable({
    providedIn: 'root'
})
export class SocialService {
    private http = inject(HttpClient);
    private baseUrl = environment.base_url + '/api/social';
    private platformId = inject(PLATFORM_ID);

    friends = signal<Friend[]>([]);
    pendingRequests = signal<Friend[]>([]);
    invitations = signal<MissionInvitation[]>([]);

    isLoadingFriends = signal(false);
    isLoadingInvitations = signal(false);

    async loadFriends() {
        if (!isPlatformBrowser(this.platformId)) return;
        this.isLoadingFriends.set(true);
        try {
            const friends = await lastValueFrom(this.http.get<Friend[]>(`${this.baseUrl}/friends`));
            this.friends.set(friends || []);
        } catch (e) {
        } finally {
            this.isLoadingFriends.set(false);
        }
    }

    async loadPendingRequests() {
        if (!isPlatformBrowser(this.platformId)) return;
        try {
            const requests = await lastValueFrom(this.http.get<Friend[]>(`${this.baseUrl}/friends/requests`));
            this.pendingRequests.set(requests || []);
        } catch (e) {
        }
    }

    async addFriend(friendId: number) {
        return lastValueFrom(this.http.post(`${this.baseUrl}/friends/add/${friendId}`, {}));
    }

    async getFriendshipStatus(otherId: number): Promise<FriendshipStatus> {
        return lastValueFrom(this.http.get<FriendshipStatus>(`${this.baseUrl}/status/${otherId}`));
    }

    async acceptFriend(friendId: number) {
        await lastValueFrom(this.http.post(`${this.baseUrl}/friends/accept/${friendId}`, {}));
        await this.loadFriends();
        await this.loadPendingRequests();
    }

    async rejectFriend(friendId: number) {
        await lastValueFrom(this.http.delete(`${this.baseUrl}/friends/reject/${friendId}`));
        await this.loadPendingRequests();
    }

    async removeFriend(friendId: number) {
        await lastValueFrom(this.http.delete(`${this.baseUrl}/friends/remove/${friendId}`));
        await this.loadFriends();
    }

    async loadInvitations() {
        if (!isPlatformBrowser(this.platformId)) return;
        this.isLoadingInvitations.set(true);
        try {
            const invitations = await lastValueFrom(this.http.get<MissionInvitation[]>(`${this.baseUrl}/invitations`));
            this.invitations.set(invitations || []);
        } catch (e) {
        } finally {
            this.isLoadingInvitations.set(false);
        }
    }

    async inviteToMission(inviteeId: number, missionId: number) {
        return lastValueFrom(this.http.post(`${this.baseUrl}/invite/${inviteeId}/${missionId}`, {}));
    }

    async respondToInvitation(invitationId: number, accept: boolean) {
        const res = await lastValueFrom(this.http.post<{ mission_id: number }>(`${this.baseUrl}/invitations/respond/${invitationId}`, { accept }));
        await this.loadInvitations();
        return res.mission_id;
    }

    async loadMissionInvitations(missionId: number): Promise<MissionInvitation[]> {
        if (!isPlatformBrowser(this.platformId)) return [];
        try {
            return await lastValueFrom(this.http.get<MissionInvitation[]>(`${this.baseUrl}/mission/${missionId}/invitations`));
        } catch (e) {
            return [];
        }
    }
}
