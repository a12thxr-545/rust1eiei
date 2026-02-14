import { Component, computed, HostListener, inject, signal, Signal } from '@angular/core';
import { FormBuilder, FormGroup, ReactiveFormsModule, Validators } from '@angular/forms';
import { NavigationEnd, Router, RouterLink, RouterLinkActive } from '@angular/router';
import { PassportService } from '../_services/passport-service';
import { MissionService } from '../_services/mission-service';
import { SnackbarService } from '../_services/snackbar.service';
import { ThemeService } from '../_services/theme.service';
import { filter } from 'rxjs';
import { SocialService } from '../_services/social-service';
import { SquadModal } from './squad-modal';
import { InboxModal } from './inbox-modal';
import { CacheManager } from '../_helpers/cache.helper';

@Component({
  selector: 'app-navbar',
  imports: [RouterLink, RouterLinkActive, ReactiveFormsModule, SquadModal, InboxModal],
  templateUrl: './navbar.html',
  styleUrl: './navbar.css',
})
export class Navbar {
  private _passport = inject(PassportService);
  private _router = inject(Router);
  public missionService = inject(MissionService);
  private _snackbar = inject(SnackbarService);
  private _fb = inject(FormBuilder);
  public themeService = inject(ThemeService);
  public socialService = inject(SocialService);

  display_name: Signal<string | undefined>
  avatar_url: Signal<string | undefined>
  isLoggedIn: Signal<boolean>
  hasNotifications: Signal<boolean>
  isOnLoginPage = signal<boolean>(false);
  showMenu = signal<boolean>(false);
  showCreateModal = signal<boolean>(false);
  showSquadModal = signal<boolean>(false);
  showInboxModal = signal<boolean>(false);
  showClearCacheConfirm = signal<boolean>(false);
  isLoading = signal<boolean>(false);
  isExpanded = signal<boolean>(false);

  form: FormGroup;

  constructor() {
    this.display_name = computed(() => this._passport.data()?.display_name);
    this.avatar_url = computed(() => this._passport.data()?.avatar_url || '/assets/defaultavtar.jpg');
    this.isLoggedIn = computed(() => this._passport.data() !== undefined);
    this.hasNotifications = computed(() =>
      this.isLoggedIn() && (this.socialService.pendingRequests().length + this.socialService.invitations().length) > 0
    );

    // Check initial route
    this.isOnLoginPage.set(this._router.url === '/login');

    // Listen for route changes
    this._router.events.pipe(
      filter(event => event instanceof NavigationEnd)
    ).subscribe((event: NavigationEnd) => {
      this.isOnLoginPage.set(event.urlAfterRedirects === '/login');
    });

    // Init form
    this.form = this._fb.group({
      name: ['', [Validators.required, Validators.minLength(3)]],
      description: [''],
      max_participants: [0, [Validators.required, Validators.min(0)]]
    });
  }

  @HostListener('document:click', ['$event'])
  onClickOutside(event: MouseEvent): void {
    const target = event.target as HTMLElement;
    if (!target.closest('.user-section')) {
      this.showMenu.set(false);
    }
    if (!target.closest('.navbar')) {
      this.isExpanded.set(false);
    }
  }

  toggleExpand(event: MouseEvent): void {
    if (this.isExpanded()) {
      return;
    }

    // When collapsed, clicking anywhere expands it
    this.isExpanded.set(true);
  }

  toggleMenu(): void {
    this.showMenu.update(v => !v);
  }

  logout(): void {
    this.showMenu.set(false);
    this._passport.removePassport();
    this._router.navigate(['/login']);
  }

  goToProfile(): void {
    this.showMenu.set(false);
    this._router.navigate(['/profile']);
  }

  goToSquad(): void {
    this.showMenu.set(false);
    this.showSquadModal.set(true);
  }

  goToInbox(): void {
    this.showMenu.set(false);
    this.showInboxModal.set(true);
  }

  openCreateModal(): void {
    this.showCreateModal.set(true);
  }

  confirmClearCache(): void {
    this.showMenu.set(false);
    this.showClearCacheConfirm.set(true);
  }

  performClearCache(): void {
    CacheManager.clear();
    this.showClearCacheConfirm.set(false);
    this._snackbar.success('Cache cleared successfully!');
    // Reload data if needed
    this.missionService.triggerRefresh();
  }

  closeCreateModal(): void {
    this.showCreateModal.set(false);
    this.form.reset();
  }

  async onSubmit(): Promise<void> {
    if (this.form.invalid) return;

    this.isLoading.set(true);
    const error = await this.missionService.createMission({
      name: this.form.value.name,
      description: this.form.value.description || undefined,
      max_participants: this.form.value.max_participants || 0
    });
    this.isLoading.set(false);

    if (error) {
      this._snackbar.error(error);
    } else {
      this._snackbar.success('Mission created successfully!');
      this.closeCreateModal();
      // Refresh current mission state
      this.missionService.getCurrentMission();
    }
  }
}
