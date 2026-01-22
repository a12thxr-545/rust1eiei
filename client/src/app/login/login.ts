import { Component, inject, signal } from '@angular/core';
import { FormControl, FormGroup, FormsModule, ReactiveFormsModule, Validators } from '@angular/forms';
import { PasswordMatchValidator, passwordValidator } from '../_helpers/passpword-vaidator';
import { MatFormFieldModule } from '@angular/material/form-field';
import { MatInputModule } from '@angular/material/input';
import { MatButtonModule } from '@angular/material/button';
import { MatCardModule } from '@angular/material/card';
import { MatProgressSpinnerModule } from '@angular/material/progress-spinner';
import { NgxSpinnerModule, NgxSpinnerService } from 'ngx-spinner';
import { PassportService } from '../_services/passport-service';
import { SnackbarService } from '../_services/snackbar.service';
import { Router } from '@angular/router';


@Component({
  selector: 'app-login',
  imports: [
    FormsModule,
    ReactiveFormsModule,
    MatFormFieldModule,
    MatInputModule,
    MatButtonModule,
    MatCardModule,
    MatProgressSpinnerModule,
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

  private usernameMinLength = 4;
  private usernameMaxLength = 10;
  private passwordMinLength = 8;
  private passwordMaxLength = 16;
  private displayNameMinLength = 3;

  mode: 'login' | 'regis' = 'login';
  isLoading = signal(false);
  form: FormGroup;

  errorMsg = {
    username: signal<string | null>(''),
    password: signal<string | null>(''),
    displayName: signal<string | null>(''),
    cf_password: signal<string | null>(''),
  }

  constructor() {
    this.form = new FormGroup({
      username: new FormControl(null, [
        Validators.required,
        Validators.minLength(this.usernameMinLength),
        Validators.maxLength(this.usernameMaxLength)
      ]),
      password: new FormControl(null, [
        Validators.required,
        passwordValidator(this.passwordMinLength, this.passwordMaxLength)
      ])
    })
  }

  toggleMode() {
    this.mode = this.mode === 'login' ? 'regis' : 'login';
    // Clear everything first
    this.clearErrors();
    this.form.reset();
    this.updateForm();
    this.resetFormState();
    // Make sure errors are cleared after updateForm
    this.clearErrors();
    console.log('After toggle - errors cleared:', {
      username: this.errorMsg.username(),
      password: this.errorMsg.password()
    });
  }

  clearErrors() {
    this.errorMsg.username.set('');
    this.errorMsg.password.set('');
    this.errorMsg.displayName.set('');
    this.errorMsg.cf_password.set('');
  }

  resetFormState() {
    // Reset form controls to untouched state
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
    } else {
      this.form.addControl('displayName', new FormControl(null, [
        Validators.required,
        Validators.minLength(this.displayNameMinLength)
      ]));
      this.form.addControl('cf_password', new FormControl(null, [Validators.required]));
      this.form.addValidators(PasswordMatchValidator('password', 'cf_password'));
    }
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
        else this.errorMsg.username.set('');
        break;

      case 'password':
        if (ctrl.hasError('required')) {
          this.errorMsg.password.set('Password is required');
        } else if (this.mode === 'regis') {
          // Show all missing requirements in register mode
          const errors: string[] = [];
          if (ctrl.hasError('invalidLength')) errors.push(`${this.passwordMinLength}-${this.passwordMaxLength} characters`);
          if (ctrl.hasError('invalidLowerCase')) errors.push('lowercase [a-z]');
          if (ctrl.hasError('invalidUpperCase')) errors.push('uppercase [A-Z]');
          if (ctrl.hasError('invalidNumeric')) errors.push('number [0-9]');
          if (ctrl.hasError('invalidSpecialChar')) errors.push('special character');

          if (errors.length > 0) {
            this.errorMsg.password.set('Password must have: ' + errors.join(', '));
          } else {
            this.errorMsg.password.set('');
          }
        } else {
          // In login mode, don't show detailed validation errors
          this.errorMsg.password.set('');
        }
        break;

      case 'displayName':
        if (ctrl.hasError('required')) this.errorMsg.displayName.set('aka is required');
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
    // Clear previous errors
    this.clearErrors();

    // Validate and show errors
    if (this.mode === 'login') {
      // Login mode: just check if fields are filled
      if (!this.form.value.username || !this.form.value.password) {
        this.snackbarService.warning('Please enter username and password');
        return;
      }
    } else {
      // Register mode: validate all fields and show specific errors
      // Validate each field
      this.updateErrorMsg('username');
      this.updateErrorMsg('displayName');
      this.updateErrorMsg('password');
      this.updateErrorMsg('cf_password');

      // Check if any field has error
      if (this.errorMsg.username() || this.errorMsg.displayName() ||
        this.errorMsg.password() || this.errorMsg.cf_password()) {
        this.snackbarService.warning('Please fix the errors above');
        return;
      }

      // Also check form validity for any other validation
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
        console.log('Login result:', error); // Debug
        if (error) {
          console.log('Login failed, showing error'); // Debug
          this.snackbarService.error('Invalid username or password');
          this.errorMsg.username.set('Invalid username or password');
          this.errorMsg.password.set('Invalid username or password');
          return;
        }
        this.snackbarService.success('Welcome back! Login successful');
      } else {
        const error = await this.passportService.register({
          username: this.form.value.username,
          password: this.form.value.password,
          display_name: this.form.value.displayName
        });
        if (error) {
          this.snackbarService.error('Username already exists');
          this.errorMsg.username.set('Username already exists');
          return;
        }
        this.snackbarService.success('Account created successfully!');
      }

      // Navigate to home on success
      this.router.navigate(['/']);
    } catch (error) {
      console.error('Auth error:', error);
      this.snackbarService.error('An unexpected error occurred. Please try again.');
    } finally {
      this.isLoading.set(false);
      this.spinnerService.hide('auth-spinner');
    }
  }
}

