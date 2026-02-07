import { inject, Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { environment } from '../../environments/environment.development';
import { MissionRatingSummary, AddRatingRequest } from '../_model/rating';
import { firstValueFrom } from 'rxjs';

@Injectable({
    providedIn: 'root',
})
export class RatingService {
    private _http = inject(HttpClient);
    private _url = environment.base_url + '/api/rating';

    async getMissionRatings(missionId: number): Promise<MissionRatingSummary | null> {
        try {
            return await firstValueFrom(this._http.get<MissionRatingSummary>(`${this._url}/${missionId}`));
        } catch (error) {
            console.error('Error fetching mission ratings:', error);
            return null;
        }
    }

    async addRating(missionId: number, rating: number, comment?: string): Promise<number | string> {
        try {
            const payload: AddRatingRequest = { rating, comment };
            return await firstValueFrom(this._http.post<number>(`${this._url}/${missionId}`, payload));
        } catch (error: any) {
            if (error.error && typeof error.error === 'string') {
                return error.error;
            }
            return error.message || 'An error occurred';
        }
    }

    async getMyRating(missionId: number): Promise<number | null> {
        try {
            return await firstValueFrom(this._http.get<number | null>(`${this._url}/${missionId}/my-rating`));
        } catch (error) {
            console.error('Error fetching my rating:', error);
            return null;
        }
    }
}
