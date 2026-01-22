import { Component, inject, signal, computed, ViewChild, ElementRef } from '@angular/core';
import { CommonModule } from '@angular/common';
import { MatDialogModule } from '@angular/material/dialog';
import { MatButtonModule } from '@angular/material/button';
import { PassportService } from '../_services/passport-service';
import { SnackbarService } from '../_services/snackbar.service';

@Component({
  selector: 'app-profile',
  imports: [CommonModule, MatDialogModule, MatButtonModule],
  templateUrl: './profile.html',
  styleUrl: './profile.css',
})
export class Profile {
  @ViewChild('fileInput') fileInput!: ElementRef<HTMLInputElement>;

  private passportService = inject(PassportService);
  private snackbarService = inject(SnackbarService);

  passport = this.passportService.data;
  isUploading = signal(false);
  previewUrl = signal<string | null>(null);

  // Default avatar URL (using UI Avatars service or a local default)
  private defaultAvatarUrl = 'https://ui-avatars.com/api/?background=3b82f6&color=fff&size=200&font-size=0.4&bold=true';

  // Computed avatar URL - use preview if available, otherwise current avatar, fallback to default
  displayAvatarUrl = computed(() => {
    const preview = this.previewUrl();
    if (preview) return preview;

    const currentAvatar = this.passport()?.avatar_url;
    if (currentAvatar) return currentAvatar;

    // Generate avatar with user's name or use default
    const name = this.passport()?.display_name || 'User';
    return `https://ui-avatars.com/api/?name=${encodeURIComponent(name)}&background=3b82f6&color=fff&size=200&font-size=0.4&bold=true`;
  });

  // Generate initials from display name
  initials = computed(() => {
    const name = this.passport()?.display_name || '';
    return name.split(' ')
      .map(word => word.charAt(0).toUpperCase())
      .slice(0, 2)
      .join('');
  });

  triggerFileInput() {
    this.fileInput.nativeElement.click();
  }

  async onFileSelected(event: Event) {
    const input = event.target as HTMLInputElement;
    const file = input.files?.[0];

    if (!file) return;

    // Validate file type - only accept JPG and PNG
    const allowedTypes = ['image/jpeg', 'image/png'];
    if (!allowedTypes.includes(file.type)) {
      this.snackbarService.error('Please select a JPG or PNG image');
      return;
    }

    // Validate file size (max 5MB)
    const maxSize = 5 * 1024 * 1024; // 5MB
    if (file.size > maxSize) {
      this.snackbarService.error('Image size must be less than 5MB');
      return;
    }

    // Create preview
    const reader = new FileReader();
    reader.onload = async (e) => {
      const base64String = e.target?.result as string;
      this.previewUrl.set(base64String);

      // Upload to server
      await this.uploadAvatar(base64String);
    };
    reader.readAsDataURL(file);

    // Reset input
    input.value = '';
  }

  private async uploadAvatar(base64String: string) {
    this.isUploading.set(true);

    try {
      // Remove the data:image/xxx;base64, prefix for the API
      const base64Data = base64String.split(',')[1] || base64String;

      const result = await this.passportService.uploadAvatar(base64Data);

      if (result) {
        this.snackbarService.success('Avatar updated successfully!');
        this.previewUrl.set(null); // Clear preview, use actual URL now
      } else {
        this.snackbarService.error('Failed to upload avatar');
        this.previewUrl.set(null); // Revert to original
      }
    } catch (error) {
      console.error('Upload error:', error);
      this.snackbarService.error('An error occurred while uploading');
      this.previewUrl.set(null);
    } finally {
      this.isUploading.set(false);
    }
  }
}
