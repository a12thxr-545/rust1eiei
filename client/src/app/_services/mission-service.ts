import { inject, Injectable } from '@angular/core';
import { HttpClient, HttpParams } from '@angular/common/http';
import { environment } from '../../environments/environment.development';
import { AddMission, Mission, MissionFilter } from '../_model/mission';
import { firstValueFrom } from 'rxjs';

@Injectable({
  providedIn: 'root',
})
export class MissionService {
  private _http = inject(HttpClient);
  private _management_url = environment.base_url + '/api/mission-management';
  private _view_url = environment.base_url + '/api/view';

  async createMission(mission: AddMission): Promise<string | null> {
    try {
      await firstValueFrom(this._http.post(this._management_url + '/', mission));
      return null;
    } catch (error: any) {
        if (error.error && typeof error.error === 'string') {
            return error.error;
        }
        return error.message || 'An error occurred';
    }
  }

  async getMissions(filter?: MissionFilter): Promise<Mission[]> {
    let params = new HttpParams();
    if (filter) {
        if (filter.name) params = params.set('name', filter.name);
        if (filter.status) params = params.set('status', filter.status);
    }
    
    return firstValueFrom(this._http.get<Mission[]>(this._view_url + '/gets', { params }));
  }
}
