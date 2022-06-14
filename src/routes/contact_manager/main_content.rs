use std::num::NonZeroU64;
use yew::prelude::*;

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub id: Option<NonZeroU64>,
}

#[function_component(MainContent)]
pub fn main_content(props: &Props) -> Html {
    html! {
        <div>
            {if let Some(id) = props.id {
                format!("User: {id}")
            } else {
                "N-O-O-O-THING!!!!!".to_owned()
            }}
        </div>
    }
}

// <div *ngIf="user$ | async as user">
//   <mat-card>
//     <mat-card-header>
//       <mat-icon mat-card-avatar [svgIcon]="user.avatar"></mat-icon>
//       <mat-card-title>
//         {{
//           'ContactManager.MainContent.NameAndGenderTitle'
//             | translate
//               : {
//                   name: user.name,
//                   gender: user.gender
//                 }
//         }}
//       </mat-card-title>
//       <mat-card-subtitle>
//         {{
//           'ContactManager.MainContent.BirthdaySubTitle'
//             | translate: { birthDate: user.birthDate }
//         }}
//       </mat-card-subtitle>
//     </mat-card-header>
//     <mat-card-content>
//       <mat-tab-group>
//         <mat-tab [label]="'ContactManager.MainContent.BioTab' | translate">
//           <p>{{ user.bio }}</p>
//         </mat-tab>
//         <mat-tab [label]="'ContactManager.MainContent.NotesTab' | translate">
//           <app-notes [notes]="user.notes"></app-notes>
//         </mat-tab>
//       </mat-tab-group>
//     </mat-card-content>
//   </mat-card>
// </div>

// import { ChangeDetectionStrategy, Component } from '@angular/core';
// import { Store } from '@ngrx/store';

// import { getCurrentUser, State } from '../../store';

// @Component({
//   selector: 'app-main-content',
//   templateUrl: './main-content.component.html',
//   styleUrls: ['./main-content.component.scss'],
//   changeDetection: ChangeDetectionStrategy.OnPush,
// })
// export class MainContentComponent {
//   readonly user$ = this.store.select(getCurrentUser);

//   constructor(private readonly store: Store<State>) {}
// }
