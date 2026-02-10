import { inject, Injectable, PLATFORM_ID, signal } from '@angular/core';
import { HttpClient, HttpParams } from '@angular/common/http';
import { isPlatformBrowser } from '@angular/common';
import { environment } from '../../environments/environment.development';
import { AddMission, CrewMember, EditMission, Mission, MissionFilter, UploadedImage } from '../_model/mission';
import { firstValueFrom } from 'rxjs';

@Injectable({
  providedIn: 'root',
})
export class MissionService {
  private _http = inject(HttpClient);
  private _management_url = environment.base_url + '/api/mission-management';
  private _view_url = environment.base_url + '/api/view';
  private _crew_url = environment.base_url + '/api/crew';
  private _operation_url = environment.base_url + '/api/mission';

  missions = signal<Mission[]>([]);
  myMissions = signal<Mission[]>([]);
  joinedMissions = signal<Mission[]>([]);
  finishedMissions = signal<Mission[]>([]);
  isLoading = signal<boolean>(false);
  isLoadingMyMissions = signal<boolean>(false);
  isLoadingFinishedMissions = signal<boolean>(false);
  currentMissionId = signal<number | null>(null);

  async startMission(missionId: number): Promise<string | null> {
    try {
      await firstValueFrom(this._http.patch(`${this._operation_url}/in-progress/${missionId}`, {}, { responseType: 'text' }));
      return null;
    } catch (error: any) {
      if (error.error && typeof error.error === 'string') {
        return error.error;
      }
      return error.message || 'An error occurred';
    }
  }

  async completeMission(missionId: number): Promise<string | null> {
    try {
      await firstValueFrom(this._http.patch(`${this._operation_url}/to-completed/${missionId}`, {}, { responseType: 'text' }));
      return null;
    } catch (error: any) {
      if (error.error && typeof error.error === 'string') {
        return error.error;
      }
      return error.message || 'An error occurred';
    }
  }

  async failMission(missionId: number): Promise<string | null> {
    try {
      await firstValueFrom(this._http.patch(`${this._operation_url}/to-failed/${missionId}`, {}, { responseType: 'text' }));
      return null;
    } catch (error: any) {
      if (error.error && typeof error.error === 'string') {
        return error.error;
      }
      return error.message || 'An error occurred';
    }
  }

  async createMission(mission: AddMission): Promise<string | null> {
    try {
      await firstValueFrom(this._http.post(this._management_url, mission));
      return null;
    } catch (error: any) {
      if (error.error && typeof error.error === 'string') {
        return error.error;
      }
      return error.message || 'An error occurred';
    }
  }

  async uploadMissionImage(base64String: string): Promise<UploadedImage | null> {
    try {
      const result = this._http.post<UploadedImage>(`${this._management_url}/image`, { base64_string: base64String });
      return await firstValueFrom(result);
    } catch (error) {
      console.error('Upload mission image error:', error);
      return null;
    }
  }

  async editMission(missionId: number, mission: EditMission): Promise<string | null> {
    try {
      await firstValueFrom(this._http.patch(`${this._management_url}/${missionId}`, mission, { responseType: 'text' }));
      return null;
    } catch (error: any) {
      if (error.error && typeof error.error === 'string') {
        return error.error;
      }
      return error.message || 'An error occurred';
    }
  }

  async deleteMission(missionId: number): Promise<string | null> {
    try {
      await firstValueFrom(this._http.delete(`${this._management_url}/${missionId}`, { responseType: 'text' }));
      return null;
    } catch (error: any) {
      if (error.error && typeof error.error === 'string') {
        return error.error;
      }
      return error.message || 'An error occurred';
    }
  }

  async joinMission(missionId: number): Promise<string | null> {
    try {
      await firstValueFrom(this._http.post(`${this._crew_url}/join/${missionId}`, {}, { responseType: 'text' }));
      return null;
    } catch (error: any) {
      if (error.error && typeof error.error === 'string') {
        return error.error;
      }
      return error.message || 'An error occurred';
    }
  }

  async leaveMission(missionId: number): Promise<string | null> {
    try {
      await firstValueFrom(this._http.delete(`${this._crew_url}/leave/${missionId}`, { responseType: 'text' }));
      return null;
    } catch (error: any) {
      if (error.error && typeof error.error === 'string') {
        return error.error;
      }
      return error.message || 'An error occurred';
    }
  }

  async kickMember(missionId: number, brawlerId: number): Promise<string | null> {
    try {
      await firstValueFrom(this._http.delete(`${this._crew_url}/kick/${missionId}/${brawlerId}`, { responseType: 'text' }));
      return null;
    } catch (error: any) {
      if (error.error && typeof error.error === 'string') {
        return error.error;
      }
      return error.message || 'An error occurred';
    }
  }

  private _platformId = inject(PLATFORM_ID);

  async getCurrentMission(): Promise<number | null> {
    if (!isPlatformBrowser(this._platformId)) return null;
    try {
      const response = await firstValueFrom(this._http.get<{ mission_id: number | null }>(`${this._crew_url}/current`));
      this.currentMissionId.set(response.mission_id);
      return response.mission_id;
    } catch (error) {
      console.error('Error getting current mission:', error);
      return null;
    }
  }

  async loadMissions(filter?: MissionFilter): Promise<void> {
    this.isLoading.set(true);
    try {
      const missions = await this.getMissions(filter);
      this.missions.set(missions);
    } catch (error) {
      console.error('Error loading missions:', error);
    } finally {
      this.isLoading.set(false);
    }
  }

  async loadOtherMissions(filter: MissionFilter): Promise<void> {
    this.isLoading.set(true);
    try {
      // Only show Open missions in Explore
      const missions = await this.getMissions({ ...filter, status: 'Open' });
      this.missions.set(missions);
    } catch (error) {
      console.error('Error loading other missions:', error);
    } finally {
      this.isLoading.set(false);
    }
  }

  async loadMyMissions(chiefId: number): Promise<void> {
    this.isLoadingMyMissions.set(true);
    try {
      // Show Open and In-Progress in My Missions
      const open = await this.getMissions({ chief_id: chiefId, status: 'Open' });
      const inProgress = await this.getMissions({ chief_id: chiefId, status: 'InProgress' });
      this.myMissions.set([...open, ...inProgress].sort((a, b) => b.id - a.id));
    } catch (error) {
      console.error('Error loading my missions:', error);
    } finally {
      this.isLoadingMyMissions.set(false);
    }
  }

  async loadJoinedMissions(memberId: number): Promise<void> {
    this.isLoadingMyMissions.set(true);
    try {
      const open = await this.getMissions({
        member_id: memberId,
        exclude_chief_id: memberId,
        status: 'Open'
      });
      const inProgress = await this.getMissions({
        member_id: memberId,
        exclude_chief_id: memberId,
        status: 'InProgress'
      });
      this.joinedMissions.set([...open, ...inProgress]);
    } catch (error) {
      console.error('Error loading joined missions:', error);
    } finally {
      this.isLoadingMyMissions.set(false);
    }
  }

  async loadFinishedMissions(userId: number): Promise<void> {
    this.isLoadingFinishedMissions.set(true);
    try {
      // Load both completed and failed missions where user was chief or member
      const completed = await this.getMissions({
        member_id: userId,
        status: 'Completed'
      });
      const failed = await this.getMissions({
        member_id: userId,
        status: 'Failed'
      });
      this.finishedMissions.set([...completed, ...failed].sort((a, b) => b.id - a.id));
    } catch (error) {
      console.error('Error loading finished missions:', error);
    } finally {
      this.isLoadingFinishedMissions.set(false);
    }
  }

  async getMissions(filter?: MissionFilter): Promise<Mission[]> {
    let params = new HttpParams();
    if (filter) {
      Object.entries(filter).forEach(([key, value]) => {
        if (value !== undefined && value !== null && value !== '') {
          params = params.set(key, value.toString());
        }
      });
    }

    return firstValueFrom(this._http.get<Mission[]>(this._view_url + '/gets', { params }));
  }

  async getMission(missionId: number): Promise<Mission | null> {
    try {
      return await firstValueFrom(this._http.get<Mission>(`${this._view_url}/${missionId}`));
    } catch (error) {
      console.error('Error fetching mission details:', error);
      return null;
    }
  }

  async getCrewMembers(missionId: number): Promise<CrewMember[]> {
    try {
      return await firstValueFrom(this._http.get<CrewMember[]>(`${this._view_url}/count/${missionId}`));
    } catch (error) {
      console.error('Error fetching crew members:', error);
      return [];
    }
  }
}
