import { Component, inject, input, OnInit } from '@angular/core';
import { User } from '../../_model/pagination';
import { MatCardModule } from '@angular/material/card';
import { MatButtonModule } from '@angular/material/button';
import { RouterLink } from '@angular/router';
import { LikeService } from '../../_services/like';
import { CacheManager } from '../../_helpers/cache.helper';
import { SocialService } from '../../_services/social-service';
import { PassportService } from '../../_services/passport-service';
import { SnackbarService } from '../../_services/snackbar.service';

@Component({
  selector: 'app-member-card',
  imports: [MatCardModule, MatButtonModule, RouterLink],
  templateUrl: './member-card.html',
  styleUrl: './member-card.css',
})
export class MemberCard implements OnInit {
  private _likeService = inject(LikeService);
  private _socialService = inject(SocialService);
  public passportService = inject(PassportService);
  private _snackbar = inject(SnackbarService);

  member = input.required<User>();
  isLikeMember = false;

  ngOnInit(): void {
    if (this.member()) {
      this.isLikeMember = this._likeService.isLikeMember(this.member().id!.toString());
    }
  }

  toggleLike() {
    if (this.member()) {
      this._likeService.toggleLike(this.member().id!.toString());
      this.isLikeMember = !this.isLikeMember;
      CacheManager.clear(); // จะให้ดีควรเข้าไปแก้ไขข้อมูล เฉพาะ member นี้ จะดีกว่าลบไปทั้งหมด
    }
  }

  async addFriend() {
    if (this.member()) {
      try {
        await this._socialService.addFriend(this.member().id!);
        this._snackbar.success('Friend request sent!');
      } catch (e: any) {
        this._snackbar.error(e.error || 'Failed to send friend request');
      }
    }
  }
}
