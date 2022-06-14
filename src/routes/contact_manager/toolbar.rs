use super::is_small_window;
use material_yew::{
    top_app_bar_fixed::{
        MatTopAppBarActionItems, MatTopAppBarNavigationIcon, MatTopAppBarTitle,
    },
    MatIconButton, MatListItem, MatMenu, MatTopAppBarFixed,
};
use yew::prelude::*;
use yew_hooks::use_bool_toggle;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub onnavigationiconclick: Callback<()>,
}

#[function_component(Toolbar)]
pub fn toolbar(props: &Props) -> Html {
    let is_menu_open = use_bool_toggle(false);

    let onclick = {
        let is_menu_open = is_menu_open.clone();
        Callback::from(move |_| is_menu_open.toggle())
    };

    let onopened = {
        let is_menu_open = is_menu_open.clone();
        Callback::from(move |_| is_menu_open.set(true))
    };

    let onclosed = {
        let is_menu_open = is_menu_open.clone();
        Callback::from(move |_| is_menu_open.set(false))
    };

    html! {
        <MatTopAppBarFixed onnavigationiconclick={&props.onnavigationiconclick}>
            {if is_small_window() {
                html! {
                    <MatTopAppBarNavigationIcon>
                        <MatIconButton icon="menu"></MatIconButton>
                    </MatTopAppBarNavigationIcon>
                }
            } else {
                html! {
                    <></>
                }
            }}
            //   <button mat-button class="sidenav-toggle" (click)="toggleSidenav.emit()">
            //     <mat-icon>menu</mat-icon>
            //   </button>

            <MatTopAppBarTitle>
                <div class="app-title">
                    // <AppLink to={AppRoute::Home}>
                    //     <h1>{"Material Yew"}</h1>
                    // </AppLink>
                    // <span class="action-item">
                    //     <AppLink to={AppRoute::Components}>
                    //         {components}
                    //     </AppLink>
                    // </span>
                </div>

                //   <span>{{ 'ContactManager.Toolbar.Title' | translate }}</span>
            </MatTopAppBarTitle>

            <MatTopAppBarActionItems>

                <div style="position: relative;">
                    <span {onclick}>
                        <MatIconButton icon="more_vert"></MatIconButton>
                    </span>
                    <MatMenu open={*is_menu_open} {onopened} {onclosed}>
                        <MatListItem>{ "New Contact" }</MatListItem>
                        <li divider="true"></li>
                        <MatListItem>{ "Toggle Theme" }</MatListItem>
                        <MatListItem>{ "Toggle Dir" }</MatListItem>
                        //     <button mat-menu-item (click)="openAddContactDialog()">
                        //       {{ 'ContactManager.Toolbar.Menu.NewContact' | translate }}
                        //     </button>
                        //     <button mat-menu-item (click)="toggleTheme.emit()">
                        //       {{ 'ContactManager.Toolbar.Menu.ToggleTheme' | translate }}
                        //     </button>
                        //     <button mat-menu-item (click)="toggleDir.emit()">
                        //       {{ 'ContactManager.Toolbar.Menu.ToggleDir' | translate }}
                        //     </button>
                    </MatMenu>
                </div>
            </MatTopAppBarActionItems>

            <div></div>
        </MatTopAppBarFixed>
    }
}

// $iconWidth: 56px;

// .example-spacer {
//   flex: 1 1 auto;
// }

// .sidenav-toggle {
//   display: none;

//   padding: 0;
//   margin: 8px;
//   min-width: $iconWidth;

//   @media (max-width: 720px) {
//     display: flex;
//     align-items: center;
//     justify-content: center;
//   }

//   mat-icon {
//     font-size: 30px;
//     height: $iconWidth;
//     width: $iconWidth;
//     line-height: $iconWidth;
//     color: white;
//   }
// }

// import {
//     ChangeDetectionStrategy,
//     Component,
//     EventEmitter,
//     Output,
//   } from '@angular/core';
//   import { MatDialog } from '@angular/material/dialog';
//   import {
//     MatSnackBar,
//     MatSnackBarRef,
//     SimpleSnackBar,
//   } from '@angular/material/snack-bar';
//   import { Router } from '@angular/router';
//   import { TranslateService } from '@ngx-translate/core';

//   import { User } from '../../models';
//   import { NewContactDialogComponent } from '../new-contact-dialog/new-contact-dialog.component';

//   @Component({
//     selector: 'app-toolbar',
//     templateUrl: './toolbar.component.html',
//     styleUrls: ['./toolbar.component.scss'],
//     changeDetection: ChangeDetectionStrategy.OnPush,
//   })
//   export class ToolbarComponent {
//     @Output() readonly toggleSidenav = new EventEmitter<void>();
//     @Output() readonly toggleTheme = new EventEmitter<void>();
//     @Output() readonly toggleDir = new EventEmitter<void>();

//     constructor(
//       private readonly dialog: MatDialog,
//       private readonly snackBar: MatSnackBar,
//       private readonly router: Router,
//       private readonly translate: TranslateService
//     ) {}

//     openAddContactDialog(): void {
//       const dialogRef = this.dialog.open(NewContactDialogComponent, {
//         width: '450px',
//       });

//       dialogRef.afterClosed().subscribe((result: User | null) => {
//         if (result) {
//           const keys = [
//             'ContactManager.Toolbar.SnackBar.Message',
//             'ContactManager.Toolbar.SnackBar.Action',
//           ];

//           this.translate.get(keys).subscribe((values: Record<string, string>) => {
//             this.openSnackBar(values[keys[0]], values[keys[1]])
//               .onAction()
//               .subscribe(() => {
//                 void this.router.navigate(['/contact-manager', result.id]);
//               });
//           });
//         }
//       });
//     }

//     openSnackBar(
//       message: string,
//       action: string
//     ): MatSnackBarRef<SimpleSnackBar> {
//       return this.snackBar.open(message, action, {
//         duration: 5000,
//       });
//     }
//   }
