// <h2 mat-dialog-title>
//   {{ 'ContactManager.NewContactDialog.Title' | translate }}
// </h2>
// <mat-dialog-content>
//   <form [formGroup]="formGroup">
//     <div class="example-container">
//       <mat-form-field appearance="fill">
//         <mat-label>
//           {{ 'ContactManager.NewContactDialog.AvatarField.Label' | translate }}
//         </mat-label>
//         <mat-select [formControl]="formGroup.controls.avatar" required>
//           <mat-select-trigger>
//             <mat-icon [svgIcon]="formGroup.controls.avatar.value"></mat-icon>
//             {{ formGroup.controls.avatar.value }}
//           </mat-select-trigger>
//           <mat-option *ngFor="let avatar of avatars" [value]="avatar">
//             <mat-icon [svgIcon]="avatar"></mat-icon>
//             {{ avatar }}
//           </mat-option>
//         </mat-select>
//       </mat-form-field>
//       <mat-form-field appearance="fill">
//         <mat-label>
//           {{ 'ContactManager.NewContactDialog.GenderField.Label' | translate }}
//         </mat-label>
//         <mat-select [formControl]="formGroup.controls.gender" required>
//           <mat-option value="male">
//             {{
//               'ContactManager.NewContactDialog.GenderField.Options.Male'
//                 | translate
//             }}
//           </mat-option>
//           <mat-option value="female">
//             {{
//               'ContactManager.NewContactDialog.GenderField.Options.Female'
//                 | translate
//             }}
//           </mat-option>
//           <mat-option value="enby">
//             {{
//               'ContactManager.NewContactDialog.GenderField.Options.NonBinary'
//                 | translate
//             }}
//           </mat-option>
//         </mat-select>
//       </mat-form-field>
//       <mat-form-field appearance="fill">
//         <mat-label>
//           {{ 'ContactManager.NewContactDialog.NameField.Label' | translate }}
//         </mat-label>
//         <input matInput [formControl]="formGroup.controls.name" required />
//         <mat-error *ngIf="formGroup.controls.name.errors as errors">
//           <ng-container *ngIf="errors['maxlength'] as maxLength">
//             {{
//               'ContactManager.NewContactDialog.NameField.Errors.MaxLength'
//                 | translate
//                   : {
//                       excessCount:
//                         maxLength.actualLength - maxLength.requiredLength
//                     }
//             }}
//           </ng-container>
//         </mat-error>
//       </mat-form-field>
//       <mat-form-field appearance="fill">
//         <mat-label>
//           {{ 'ContactManager.NewContactDialog.BornField.Label' | translate }}
//         </mat-label>
//         <input
//           [formControl]="formGroup.controls.birthDate"
//           matInput
//           [matDatepicker]="picker"
//           [min]="minBirthDate"
//           [max]="maxBirthDate"
//           required
//         />
//         <mat-datepicker-toggle matSuffix [for]="picker"></mat-datepicker-toggle>
//         <mat-datepicker #picker></mat-datepicker>
//         <mat-error *ngIf="formGroup.controls.birthDate.errors as errors">
//           <ng-container *ngIf="errors['y2k']">
//             {{
//               'ContactManager.NewContactDialog.BornField.Errors.Y2K' | translate
//             }}
//           </ng-container>
//           <ng-container *ngIf="errors['year2012']">
//             {{
//               'ContactManager.NewContactDialog.BornField.Errors.Year2012'
//                 | translate
//             }}
//           </ng-container>
//         </mat-error>
//       </mat-form-field>
//       <mat-form-field appearance="fill">
//         <mat-label>
//           {{ 'ContactManager.NewContactDialog.BioField.Label' | translate }}
//         </mat-label>
//         <textarea
//           matInput
//           [formControl]="formGroup.controls.bio"
//           rows="10"
//           cols="30"
//         >
//           <!-- [maxlength]="bioMaxLength">!--> <!-- NOTE: Doesn't allow newlines!-->
//         </textarea>
//         <mat-error *ngIf="formGroup.controls.bio.errors as errors">
//           <ng-container *ngIf="errors['maxlength'] as maxLength">
//             {{
//               'ContactManager.NewContactDialog.BioField.Errors.MaxLength'
//                 | translate
//                   : {
//                       excessCount:
//                         maxLength.actualLength - maxLength.requiredLength
//                     }
//             }}
//           </ng-container>
//         </mat-error>
//       </mat-form-field>
//     </div>
//   </form>
// </mat-dialog-content>
// <mat-dialog-actions>
//   <button
//     mat-button
//     color="primary"
//     (click)="save()"
//     [disabled]="!formGroup.valid"
//   >
//     <mat-icon>save</mat-icon>
//     {{ 'ContactManager.NewContactDialog.Save' | translate }}
//   </button>
//   <button mat-button color="primary" (click)="dismiss()">
//     <mat-icon>cancel</mat-icon>
//     {{ 'ContactManager.NewContactDialog.Cancel' | translate }}
//   </button>
// </mat-dialog-actions>

// .example-container {
//     display: flex;
//     flex-direction: column;
//   }

//   .example-container > * {
//     width: 100%;
//   }


//   import { ChangeDetectionStrategy, Component, OnInit } from '@angular/core';
//   import { FormControl, Validators } from '@angular/forms';
//   import { MatDialogRef } from '@angular/material/dialog';
//   import { UntilDestroy, untilDestroyed } from '@ngneat/until-destroy';
//   import { Actions, ofType } from '@ngrx/effects';
//   import { Store } from '@ngrx/store';
//   import { typedFormGroup } from 'ngx-forms-typed';

//   import { User } from '../../models';
//   import { createUser, createUserSuccess, State } from '../../store';
//   import { y2kValidator, year2012Validator } from 'src/app/utils';

//   @UntilDestroy()
//   @Component({
//     selector: 'app-new-contact-dialog',
//     templateUrl: './new-contact-dialog.component.html',
//     styleUrls: ['./new-contact-dialog.component.scss'],
//     changeDetection: ChangeDetectionStrategy.OnPush,
//   })
//   export class NewContactDialogComponent implements OnInit {
//     static readonly nameMaxLength = 20;
//     static readonly bioMaxLength = 30;

//     readonly avatars = ['svg-1', 'svg-2', 'svg-3', 'svg-4'];
//     readonly minBirthDate = new Date('1970-01-01Z00:00:00:000');
//     readonly maxBirthDate = new Date();
//     readonly formGroup = typedFormGroup<User>({
//       id: new FormControl(null),
//       birthDate: new FormControl(null, [
//         Validators.required,
//         y2kValidator,
//         year2012Validator,
//       ]),
//       gender: new FormControl(null, [Validators.required]),
//       name: new FormControl('', [
//         Validators.required,
//         Validators.maxLength(NewContactDialogComponent.nameMaxLength),
//       ]),
//       avatar: new FormControl(null, [Validators.required]),
//       bio: new FormControl('', [
//         Validators.maxLength(NewContactDialogComponent.bioMaxLength),
//       ]),
//       notes: new FormControl([]),
//     });

//     constructor(
//       private readonly dialogRef: MatDialogRef<NewContactDialogComponent>,
//       private readonly store: Store<State>,
//       private readonly actions$: Actions
//     ) {}

//     ngOnInit(): void {
//       this.actions$
//         .pipe(ofType(createUserSuccess), untilDestroyed(this))
//         .subscribe(({ user }) => {
//           this.dialogRef.close(user);
//         });
//     }

//     save(): void {
//       if (this.formGroup.valid) {
//         this.store.dispatch(createUser({ user: this.formGroup.value }));
//       }
//     }

//     dismiss(): void {
//       this.dialogRef.close(null);
//     }
//   }
