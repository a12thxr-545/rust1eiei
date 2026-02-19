import { computed, inject, Injectable, Signal } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { environment } from '../../environments/environment';
import { PassportService } from './passport-service';
import { User } from '../_model/pagination';

export interface UserWithFollowing {
  id: string;
  following?: string[];
}

@Injectable({
  providedIn: 'root',
})
export class LikeService {
  private _http = inject(HttpClient);
  private _passportService = inject(PassportService);
  private _apiUrl = environment.base_url + '/api/like/';

  user: Signal<UserWithFollowing | undefined>;

  constructor() {
    this.user = computed(() => {
      const passport = this._passportService.data();
      if (passport) {
        return {
          id: passport.display_name, // Using display_name as user identifier
          following: [] // This would come from the actual user data
        } as UserWithFollowing;
      }
      return undefined;
    });
  }

  isLikeMember(member_id: string): boolean {
    const ListOfLikeMembers = this.user()?.following as string[] || [];
    const isLike = ListOfLikeMembers.includes(member_id);
    return !!isLike;
  }

  toggleLike(member_id: string) {
    const user = this.user();
    if (!user) return;

    this._http.post(this._apiUrl + member_id, {}).subscribe({
      next: () => {
        console.log('Toggled like for member:', member_id);
      },
      error: (err) => {
        console.error('Error toggling like:', err);
      }
    });
  }
}
