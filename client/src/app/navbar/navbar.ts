import { Component, computed, HostListener, inject, signal, Signal } from '@angular/core';
import { FormBuilder, FormGroup, ReactiveFormsModule, Validators } from '@angular/forms';
import { NavigationEnd, Router, RouterLink, RouterLinkActive } from '@angular/router';
import { PassportService } from '../_services/passport-service';
import { MissionService } from '../_services/mission-service';
import { SnackbarService } from '../_services/snackbar.service';
import { ThemeService } from '../_services/theme.service';
import { LanguageService } from '../_services/language.service';
import { filter } from 'rxjs';

@Component({
  selector: 'app-navbar',
  imports: [RouterLink, RouterLinkActive, ReactiveFormsModule],
  templateUrl: './navbar.html',
  styleUrl: './navbar.css',
})
export class Navbar {
  private _passport = inject(PassportService);
  private _router = inject(Router);
  private _missionService = inject(MissionService);
  private _snackbar = inject(SnackbarService);
  private _fb = inject(FormBuilder);
  public themeService = inject(ThemeService);
  public lang = inject(LanguageService);

  display_name: Signal<string | undefined>
  avatar_url: Signal<string | undefined>
  isLoggedIn: Signal<boolean>
  isOnLoginPage = signal<boolean>(false);
  showMenu = signal<boolean>(false);
  showCreateModal = signal<boolean>(false);
  isLoading = signal<boolean>(false);

  form: FormGroup;

  constructor() {
    this.display_name = computed(() => this._passport.data()?.display_name);
    this.avatar_url = computed(() => this._passport.data()?.avatar_url || '/assets/defaultavtar.jpg');
    this.isLoggedIn = computed(() => this._passport.data() !== undefined);

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
    });
  }

  @HostListener('document:click', ['$event'])
  onClickOutside(event: MouseEvent): void {
    const target = event.target as HTMLElement;
    if (!target.closest('.user-section')) {
      this.showMenu.set(false);
    }
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

  openCreateModal(): void {
    this.showCreateModal.set(true);
  }

  closeCreateModal(): void {
    this.showCreateModal.set(false);
    this.form.reset();
  }

  async onSubmit(): Promise<void> {
    if (this.form.invalid) return;

    this.isLoading.set(true);
    const error = await this._missionService.createMission({
      name: this.form.value.name,
      description: this.form.value.description || undefined,
    });
    this.isLoading.set(false);

    if (error) {
      this._snackbar.error(error);
    } else {
      this._snackbar.success('Mission created successfully!');
      this.closeCreateModal();
      // Refresh current mission state
      this._missionService.getCurrentMission();
    }
  }
}
