// <div class="mat-elevation-z8">
//   <mat-form-field>
//     <mat-label> {{ 'ContactManager.Notes.Filter' | translate }} </mat-label>
//     <input
//       #filter
//       matInput
//       (keyup)="applyFilter($event)"
//       [placeholder]="'ContactManager.Notes.Filter' | translate"
//     />
//   </mat-form-field>

//   <table mat-table matSort [dataSource]="dataSource">
//     <ng-container matColumnDef="id">
//       <th mat-header-cell *matHeaderCellDef mat-sort-header>
//         {{ 'ContactManager.Notes.NumberColumn' | translate }}
//       </th>
//       <td mat-cell *matCellDef="let note">{{ note.id }}</td>
//     </ng-container>

//     <ng-container matColumnDef="title">
//       <th mat-header-cell *matHeaderCellDef mat-sort-header>
//         {{ 'ContactManager.Notes.TitleColumn' | translate }}
//       </th>
//       <td mat-cell *matCellDef="let note">{{ note.title }}</td>
//     </ng-container>

//     <ng-container matColumnDef="date">
//       <th mat-header-cell *matHeaderCellDef mat-sort-header>
//         {{ 'ContactManager.Notes.DateColumn.Title' | translate }}
//       </th>
//       <td mat-cell *matCellDef="let note">
//         {{
//           'ContactManager.Notes.DateColumn.DateFormat'
//             | translate: { value: note.date }
//         }}
//       </td>
//     </ng-container>

//     <tr mat-header-row *matHeaderRowDef="displayedColumns"></tr>
//     <tr mat-row *matRowDef="let row; columns: displayedColumns"></tr>

//     <!-- Row shown when there is no matching data. -->
//     <tr class="mat-row" *matNoDataRow>
//       <td class="mat-cell" colspan="4">
//         {{
//           'ContactManager.Notes.NoMatchingDataForFilter'
//             | translate
//               : {
//                   filterValue: filter.value
//                 }
//         }}
//       </td>
//     </tr>
//   </table>

//   <mat-paginator
//     [pageSize]="2"
//     [pageSizeOptions]="[5, 10, 20]"
//     showFirstLastButtons
//   ></mat-paginator>
// </div>

// /* Structure */
// table {
//   width: 100%;
// }

// .mat-form-field {
//   font-size: 14px;
//   width: 100%;
// }

// import {
//     AfterViewInit,
//     ChangeDetectionStrategy,
//     Component,
//     Input,
//     ViewChild,
//   } from '@angular/core';
//   import { MatPaginator } from '@angular/material/paginator';
//   import { MatSort } from '@angular/material/sort';
//   import { MatTableDataSource } from '@angular/material/table';

//   import { Note } from '../../models';

//   @Component({
//     selector: 'app-notes',
//     templateUrl: './notes.component.html',
//     styleUrls: ['./notes.component.scss'],
//     changeDetection: ChangeDetectionStrategy.OnPush,
//   })
//   export class NotesComponent implements AfterViewInit {
//     readonly dataSource = new MatTableDataSource<Note>();

//     displayedColumns = ['id', 'title', 'date'];

//     @ViewChild(MatSort) sort?: MatSort;
//     @ViewChild(MatPaginator) paginator?: MatPaginator;

//     @Input() set notes(value: Note[]) {
//       this.dataSource.data = value;
//     }

//     ngAfterViewInit(): void {
//       this.dataSource.sort = this.sort ?? null;
//       this.dataSource.paginator = this.paginator ?? null;
//     }

//     applyFilter(event: KeyboardEvent): void {
//       const filterValue = (event.target as HTMLInputElement).value;
//       this.dataSource.filter = filterValue.trim().toLowerCase();
//     }
//   }
