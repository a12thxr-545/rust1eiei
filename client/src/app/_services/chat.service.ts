import { inject, Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { environment } from '../../environments/environment.development';
import { ChatMessage, SendMessage } from '../_model/chat';
import { firstValueFrom } from 'rxjs';

@Injectable({
    providedIn: 'root',
})
export class ChatService {
    private _http = inject(HttpClient);
    private _url = environment.base_url + '/api/chat';

    async getMessages(missionId: number): Promise<ChatMessage[]> {
        try {
            return await firstValueFrom(this._http.get<ChatMessage[]>(`${this._url}/${missionId}`));
        } catch (error) {
            console.error('Error fetching chat messages:', error);
            return [];
        }
    }

    async sendMessage(missionId: number, content: string, imageUrl?: string): Promise<number | string> {
        try {
            const payload: SendMessage = { content, imageUrl };
            return await firstValueFrom(this._http.post<number>(`${this._url}/${missionId}`, payload));
        } catch (error: any) {
            if (error.error && typeof error.error === 'string') {
                return error.error;
            }
            return error.message || 'An error occurred';
        }
    }
}

