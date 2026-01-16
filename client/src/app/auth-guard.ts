import { CanActivateFn } from '@angular/router';
import { PassportService } from './_services/passport-service';
import { inject } from '@angular/core';
import { Router } from '@angular/router';

export const authGuard: CanActivateFn = (route, state) => {

  const passport = inject(PassportService);
  const router = inject(Router);

  // Check if user is logged in (has passport data)
  if (passport.data()) {
    return true;
  }

  // Redirect to login if not authenticated
  router.navigate(['/not-found']);
  return false;
};
