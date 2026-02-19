import { Component, inject, signal, computed, ViewChild, ElementRef } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { Location } from '@angular/common';
import { PassportService } from '../_services/passport-service';
import { SnackbarService } from '../_services/snackbar.service';

@Component({
  selector: 'app-profile',
  imports: [CommonModule, FormsModule],
  templateUrl: './profile.html',
  styleUrl: './profile.css',
})
export class Profile {
  @ViewChild('fileInput') fileInput!: ElementRef<HTMLInputElement>;
  @ViewChild('coverInput') coverInput!: ElementRef<HTMLInputElement>;

  private passportService = inject(PassportService);
  private snackbarService = inject(SnackbarService);
  private location = inject(Location);

  goBack(): void {
    this.location.back();
  }

  passport = this.passportService.data;
  isUploading = signal(false);
  isUploadingCover = signal(false);
  previewUrl = signal<string | null>(null);
  coverPreviewUrl = signal<string | null>(null);

  // Display name editing
  isEditingName = signal(false);
  editDisplayName = signal('');
  isSavingName = signal(false);

  // Bio editing
  isEditingBio = signal(false);
  editBio = signal('');
  isSavingBio = signal(false);

  // Default cover gradient
  private defaultCoverGradient = 'linear-gradient(135deg, #1a3a4a 0%, #0d2a35 50%, #0a1f28 100%)';

  // Computed avatar URL
  displayAvatarUrl = computed(() => {
    const preview = this.previewUrl();
    if (preview) return preview;

    const currentAvatar = this.passport()?.avatar_url;
    if (currentAvatar) return currentAvatar;

    const name = this.passport()?.display_name || 'User';
    return `https://ui-avatars.com/api/?name=${encodeURIComponent(name)}&background=3b82f6&color=fff&size=200&font-size=0.4&bold=true`;
  });

  // Computed cover URL
  displayCoverUrl = computed(() => {
    const preview = this.coverPreviewUrl();
    if (preview) return `url(${preview})`;

    const currentCover = this.passport()?.cover_url;
    if (currentCover) return `url(${currentCover})`;

    return this.defaultCoverGradient;
  });

  triggerFileInput() {
    this.fileInput.nativeElement.click();
  }

  triggerCoverInput() {
    this.coverInput.nativeElement.click();
  }

  async onFileSelected(event: Event) {
    const input = event.target as HTMLInputElement;
    const file = input.files?.[0];

    if (!file) return;

    const allowedTypes = ['image/jpeg', 'image/png'];
    if (!allowedTypes.includes(file.type)) {
      this.snackbarService.error('Please select a JPG or PNG image');
      return;
    }

    const maxSize = 5 * 1024 * 1024;
    if (file.size > maxSize) {
      this.snackbarService.error('Image size must be less than 5MB');
      return;
    }

    const reader = new FileReader();
    reader.onload = async (e) => {
      const base64String = e.target?.result as string;
      this.previewUrl.set(base64String);
      await this.uploadAvatar(base64String);
    };
    reader.readAsDataURL(file);
    input.value = '';
  }

  async onCoverSelected(event: Event) {
    const input = event.target as HTMLInputElement;
    const file = input.files?.[0];

    if (!file) return;

    const allowedTypes = ['image/jpeg', 'image/png'];
    if (!allowedTypes.includes(file.type)) {
      this.snackbarService.error('Please select a JPG or PNG image');
      return;
    }

    const maxSize = 5 * 1024 * 1024;
    if (file.size > maxSize) {
      this.snackbarService.error('Image size must be less than 5MB');
      return;
    }

    const reader = new FileReader();
    reader.onload = async (e) => {
      const base64String = e.target?.result as string;
      this.coverPreviewUrl.set(base64String);
      await this.uploadCover(base64String);
    };
    reader.readAsDataURL(file);
    input.value = '';
  }

  private async uploadAvatar(base64String: string) {
    this.isUploading.set(true);

    try {
      const base64Data = base64String.split(',')[1] || base64String;
      const result = await this.passportService.uploadAvatar(base64Data);

      if (result) {
        this.snackbarService.success('Avatar updated!');
        this.previewUrl.set(null);
      } else {
        this.snackbarService.error('Failed to upload avatar');
        this.previewUrl.set(null);
      }
    } catch (error) {
      console.error('Upload error:', error);
      this.snackbarService.error('Upload failed');
      this.previewUrl.set(null);
    } finally {
      this.isUploading.set(false);
    }
  }

  private async uploadCover(base64String: string) {
    this.isUploadingCover.set(true);

    try {
      const base64Data = base64String.split(',')[1] || base64String;
      const result = await this.passportService.uploadCover(base64Data);

      if (result) {
        this.snackbarService.success('Cover updated!');
        this.coverPreviewUrl.set(null);
      } else {
        this.snackbarService.error('Failed to upload cover');
        this.coverPreviewUrl.set(null);
      }
    } catch (error) {
      console.error('Upload error:', error);
      this.snackbarService.error('Upload failed');
      this.coverPreviewUrl.set(null);
    } finally {
      this.isUploadingCover.set(false);
    }
  }

  // Display name editing methods
  startEditingName() {
    this.editDisplayName.set(this.passport()?.display_name || '');
    this.isEditingName.set(true);
  }

  cancelEditingName() {
    this.isEditingName.set(false);
    this.editDisplayName.set('');
  }

  async saveDisplayName() {
    const newName = this.editDisplayName().trim();
    if (!newName) {
      this.snackbarService.error('Display name cannot be empty');
      return;
    }

    this.isSavingName.set(true);
    const error = await this.passportService.updateDisplayName(newName);
    this.isSavingName.set(false);

    if (error) {
      this.snackbarService.error(error);
    } else {
      this.snackbarService.success('Display name updated!');
      this.isEditingName.set(false);
    }
  }

  // Bio editing methods
  startEditingBio() {
    this.editBio.set(this.passport()?.bio || '');
    this.isEditingBio.set(true);
  }

  cancelEditingBio() {
    this.isEditingBio.set(false);
    this.editBio.set('');
  }

  async saveBio() {
    const newBio = this.editBio().trim();

    this.isSavingBio.set(true);
    const result = await this.passportService.updateBio(newBio);
    this.isSavingBio.set(false);

    if (result) {
      this.snackbarService.success('Bio updated!');
      this.isEditingBio.set(false);
    } else {
      this.snackbarService.error('Failed to update bio');
    }
  }
}
