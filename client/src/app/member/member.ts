import { Component, inject, OnInit } from '@angular/core';
import { MemberService } from '../_services/member.service';
import { MatPaginatorModule, PageEvent } from '@angular/material/paginator';
import { FormsModule } from '@angular/forms';
import { MemberCard } from './member-card/member-card';
import { default_pagination } from '../_model/pagination';

@Component({
  selector: 'app-member',
  imports: [
    MatPaginatorModule,
    FormsModule,
    MemberCard
  ],
  templateUrl: './member.html',
  styleUrl: './member.css',
})
export class Member implements OnInit {
  private _memberService = inject(MemberService);

  pageSizeOptions = [8, 16, 24, 48];
  paginator = this._memberService.paginator;

  ngOnInit(): void {
    const paginator = this.paginator();
    paginator.pagination.pageSize = 8; // Default page size for the grid
    this._memberService.paginator.set(paginator);
    this._memberService.getMember();
  }

  onChangedPage(event: PageEvent) {
    const paginator = this.paginator();
    paginator.pagination.currentPage = event.pageIndex + 1;
    paginator.pagination.pageSize = event.pageSize;
    this._memberService.paginator.set(paginator);
    this.onSearch();
  }

  onSearch() {
    this._memberService.getMember();
  }

  resetSearch() {
    this._memberService.paginator.set(default_pagination);
    this.onSearch();
  }
}
