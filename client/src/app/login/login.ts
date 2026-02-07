import { Component, inject, PLATFORM_ID, signal } from '@angular/core';
import { isPlatformBrowser } from '@angular/common';
import { FormControl, FormGroup, FormsModule, ReactiveFormsModule, Validators } from '@angular/forms';
import { PasswordMatchValidator, passwordValidator } from '../_helpers/passpword-vaidator';
import { NgxSpinnerModule, NgxSpinnerService } from 'ngx-spinner';
import { PassportService } from '../_services/passport-service';
import { SnackbarService } from '../_services/snackbar.service';
import { Router } from '@angular/router';

@Component({
  selector: 'app-login',
  imports: [
    FormsModule,
    ReactiveFormsModule,
    NgxSpinnerModule
  ],
  templateUrl: './login.html',
  styleUrl: './login.css',
})
export class Login {
  private passportService = inject(PassportService);
  private snackbarService = inject(SnackbarService);
  private spinnerService = inject(NgxSpinnerService);
  private router = inject(Router);

  private usernameMinLength = 1;
  private usernameMaxLength = 32;
  private passwordMinLength = 8;
  private passwordMaxLength = 16;
  private displayNameMinLength = 1;

  mode: 'login' | 'regis' = 'login';
  isLoading = signal(false);
  form: FormGroup;

  errorMsg = {
    username: signal<string | null>(''),
    password: signal<string | null>(''),
    displayName: signal<string | null>(''),
    cf_password: signal<string | null>(''),
  }

  suggestedUsernames = signal<string[]>([]);

  constructor() {
    const platformId = inject(PLATFORM_ID);
    // If already logged in, redirect to home
    if (isPlatformBrowser(platformId) && this.passportService.data()) {
      this.router.navigate(['/']);
    }

    this.form = new FormGroup({
      username: new FormControl(null, [
        Validators.required,
        Validators.minLength(this.usernameMinLength),
        Validators.maxLength(this.usernameMaxLength)
      ]),
      password: new FormControl(null, [
        Validators.required
      ])
    })
  }

  toggleMode() {
    this.mode = this.mode === 'login' ? 'regis' : 'login';
    this.clearErrors();
    this.form.reset();
    this.updateForm();
    this.resetFormState();
    this.clearErrors();
  }

  clearErrors() {
    this.errorMsg.username.set('');
    this.errorMsg.password.set('');
    this.errorMsg.displayName.set('');
    this.errorMsg.cf_password.set('');
  }

  resetFormState() {
    Object.keys(this.form.controls).forEach(key => {
      const control = this.form.get(key);
      if (control) {
        control.markAsUntouched();
        control.markAsPristine();
      }
    });
  }

  updateForm() {
    if (this.mode === 'login') {
      this.form.removeControl('cf_password');
      this.form.removeControl('displayName');
      this.form.removeValidators(PasswordMatchValidator('password', 'cf_password'));
      this.form.get('password')?.setValidators([Validators.required]);
    } else {
      this.form.addControl('displayName', new FormControl(null, [
        Validators.required,
        Validators.minLength(this.displayNameMinLength)
      ]));
      this.form.addControl('cf_password', new FormControl(null, [Validators.required]));
      this.form.addValidators(PasswordMatchValidator('password', 'cf_password'));
      this.form.get('password')?.setValidators([
        Validators.required,
        passwordValidator(this.passwordMinLength, this.passwordMaxLength)
      ]);
    }
    this.form.get('password')?.updateValueAndValidity();
    this.form.updateValueAndValidity();
  }

  updateErrorMsg(ctrlName: string): void | null {
    const ctrl = this.form.controls[ctrlName];
    if (!ctrl) return null;

    switch (ctrlName) {
      case 'username':
        if (ctrl.hasError('required')) this.errorMsg.username.set('Username is required');
        else if (ctrl.hasError('minlength')) this.errorMsg.username.set(`Must be at least ${this.usernameMinLength} characters`);
        else if (ctrl.hasError('maxlength')) this.errorMsg.username.set(`Must be at most ${this.usernameMaxLength} characters`);
        else if (ctrl.hasError('pattern')) this.errorMsg.username.set('Invalid characters');
        else this.errorMsg.username.set('');
        break;

      case 'password':
        if (ctrl.hasError('required')) {
          this.errorMsg.password.set('Password is required');
        } else if (this.mode === 'regis') {
          const errors: string[] = [];
          if (ctrl.hasError('invalidLength')) errors.push(`${this.passwordMinLength}-${this.passwordMaxLength} chars`);
          if (ctrl.hasError('invalidLowerCase')) errors.push('lowercase');
          if (ctrl.hasError('invalidUpperCase')) errors.push('uppercase');
          if (ctrl.hasError('invalidNumeric')) errors.push('number');
          if (ctrl.hasError('invalidSpecialChar')) errors.push('special char');

          if (errors.length > 0) {
            this.errorMsg.password.set('Needs: ' + errors.join(', '));
          } else {
            this.errorMsg.password.set('');
          }
        } else {
          this.errorMsg.password.set('');
        }
        break;

      case 'displayName':
        if (ctrl.hasError('required')) this.errorMsg.displayName.set('Display name is required');
        else if (ctrl.hasError('minlength')) this.errorMsg.displayName.set(`Must be at least ${this.displayNameMinLength} characters`);
        else this.errorMsg.displayName.set('');
        break;

      case 'cf_password':
        if (ctrl.hasError('required')) this.errorMsg.cf_password.set('Please confirm your password');
        else if (ctrl.hasError('mismatch')) this.errorMsg.cf_password.set('Passwords do not match');
        else this.errorMsg.cf_password.set('');
        break;
    }
  }

  async onSubmit() {
    this.clearErrors();

    if (this.mode === 'login') {
      if (!this.form.value.username || !this.form.value.password) {
        this.snackbarService.warning('Please enter username and password');
        return;
      }
    } else {
      this.updateErrorMsg('username');
      this.updateErrorMsg('displayName');
      this.updateErrorMsg('password');
      this.updateErrorMsg('cf_password');

      if (this.errorMsg.username() || this.errorMsg.displayName() ||
        this.errorMsg.password() || this.errorMsg.cf_password()) {
        this.snackbarService.warning('Please fix the errors above');
        return;
      }

      if (this.form.invalid) return;
    }

    this.isLoading.set(true);
    this.spinnerService.show('auth-spinner');

    try {
      if (this.mode === 'login') {
        const error = await this.passportService.get({
          username: this.form.value.username,
          password: this.form.value.password
        });
        if (error) {
          this.snackbarService.error('Invalid username or password');
          this.errorMsg.username.set('Invalid credentials');
          this.errorMsg.password.set('Invalid credentials');
          return;
        }
        this.snackbarService.success('Welcome back!');
      } else {
        const error = await this.passportService.register({
          username: this.form.value.username,
          password: this.form.value.password,
          display_name: this.form.value.displayName
        });
        if (error) {
          const msg = error.includes('already taken') ? 'Username already taken' : 'Registration failed';
          this.snackbarService.error(msg);
          if (error.includes('already taken')) {
            this.errorMsg.username.set('Username already taken');
          }
          return;
        }
        this.snackbarService.success('Account created!');
      }

      this.router.navigate(['/']);
    } catch (error) {
      console.error('Auth error:', error);
      this.snackbarService.error('An unexpected error occurred');
    } finally {
      this.isLoading.set(false);
      this.spinnerService.hide('auth-spinner');
    }
  }

  async checkUsernameAvailability() {
    const ctrl = this.form.get('username');
    const username = ctrl?.value;
    if (!username || ctrl?.invalid) {
      this.suggestedUsernames.set([]);
      return;
    }

    const isAvailable = await this.passportService.checkUsername(username);
    if (!isAvailable) {
      this.errorMsg.username.set('Username already taken');
      this.generateSuggestions(username);
    } else {
      this.errorMsg.username.set('');
      this.suggestedUsernames.set([]);
    }
  }

  private generateSuggestions(username: string) {
    const random = () => Math.floor(Math.random() * 999);
    const suggestions = [
      `${username}${random()}`,
      `${username}_${random()}`,
      `${username}${new Date().getFullYear()}`
    ];
    this.suggestedUsernames.set(suggestions);
  }

  applySuggestion(suggestion: string) {
    this.form.patchValue({ username: suggestion });
    this.errorMsg.username.set('');
    this.suggestedUsernames.set([]);
  }
}
