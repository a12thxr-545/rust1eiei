import { Component, inject, signal } from '@angular/core';
import { FormControl, FormGroup, FormsModule, ReactiveFormsModule, Validators } from '@angular/forms';
import { PasswordMatchValidator, passwordValidator } from '../_helpers/passpword-vaidator';
import { MatFormFieldModule } from '@angular/material/form-field';
import { MatInputModule } from '@angular/material/input';
import { MatButtonModule } from '@angular/material/button';
import { MatCardModule } from '@angular/material/card';
import { PassportService } from '../_services/passport-service';
import { Router } from '@angular/router';


@Component({
  selector: 'app-login',
  imports: [FormsModule, ReactiveFormsModule, MatFormFieldModule, MatInputModule, MatButtonModule, MatCardModule],
  templateUrl: './login.html',
  styleUrl: './login.css',
})
export class Login {
  private passportService = inject(PassportService);
  private router = inject(Router);

  private usernameMinLength = 4;
  private usernameMaxLength = 10;
  private passwordMinLength = 8;
  private passwordMaxLength = 16;
  private displayNameMinLength = 3;

  mode: 'login' | 'regis' = 'login';
  isLoading = false;
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
    this.updateForm();
    this.clearErrors();
  }

  clearErrors() {
    this.errorMsg.username.set('');
    this.errorMsg.password.set('');
    this.errorMsg.displayName.set('');
    this.errorMsg.cf_password.set('');
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
        if (ctrl.hasError('required')) this.errorMsg.password.set('Password is required');
        else if (ctrl.hasError('invalidLength')) this.errorMsg.password.set(`Must be ${this.passwordMinLength}-${this.passwordMaxLength} characters`);
        else if (ctrl.hasError('invalidLowerCase')) this.errorMsg.password.set('Must contain at least 1 lowercase letter [a-z]');
        else if (ctrl.hasError('invalidUpperCase')) this.errorMsg.password.set('Must contain at least 1 uppercase letter [A-Z]');
        else if (ctrl.hasError('invalidNumeric')) this.errorMsg.password.set('Must contain at least 1 number [0-9]');
        else if (ctrl.hasError('invalidSpecialChar')) this.errorMsg.password.set('Must contain at least 1 special character');
        else this.errorMsg.password.set('');
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
    if (this.form.invalid) return;

    this.isLoading = true;

    try {
      if (this.mode === 'login') {
        const error = await this.passportService.get({
          username: this.form.value.username,
          password: this.form.value.password
        });
        if (error) {
          this.errorMsg.password.set('Invalid username or password');
          return;
        }
      } else {
        const error = await this.passportService.register({
          username: this.form.value.username,
          password: this.form.value.password,
          display_name: this.form.value.displayName
        });
        if (error) {
          this.errorMsg.username.set('Username already exists');
          return;
        }
      }

      // Navigate to home on success
      this.router.navigate(['/']);
    } catch (error) {
      console.error('Auth error:', error);
    } finally {
      this.isLoading = false;
    }
  }
}
