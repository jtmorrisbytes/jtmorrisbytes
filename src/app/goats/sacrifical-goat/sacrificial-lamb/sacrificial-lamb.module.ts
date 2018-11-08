import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { GreenLambComponent } from './green-lamb/green-lamb.component';
import { RedLambComponent } from './red-lamb/red-lamb.component';

@NgModule({
  imports: [
    CommonModule
  ],
  declarations: [GreenLambComponent, RedLambComponent]
})
export class SacrificialLambModule { }
