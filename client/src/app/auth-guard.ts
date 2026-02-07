import { CanActivateFn, Router } from '@angular/router';
import { PassportService } from './_services/passport-service';
import { inject, PLATFORM_ID } from '@angular/core';
import { isPlatformBrowser } from '@angular/common';

export const authGuard: CanActivateFn = (route, state) => {
  const passport = inject(PassportService);
  const router = inject(Router);
  const platformId = inject(PLATFORM_ID);

  // If on server, assume true and let client-side handle it after hydration
  if (!isPlatformBrowser(platformId)) {
    return true;
  }

  // Check if user is logged in (has passport data)
  let data = passport.data();
  if (!data && isPlatformBrowser(platformId)) {
    passport.loadPassportFromLocalStorage();
    data = passport.data();
  }

  if (data) {
    return true;
  }

  // Redirect to login if not authenticated
  router.navigate(['/login']);
  return false;
};
