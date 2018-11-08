import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { BlackBabyGoatComponent } from './black-baby-goat/black-baby-goat.component';
import { WhiteBabyGoatComponent } from './white-baby-goat/white-baby-goat.component';
import { SacrificialLambModule } from './sacrificial-lamb/sacrificial-lamb.module';

@NgModule({
  imports: [
    CommonModule,
    SacrificialLambModule
  ],
  declarations: [
    BlackBabyGoatComponent,
    WhiteBabyGoatComponent
  ],
  exports:[
    BlackBabyGoatComponent,
    WhiteBabyGoatComponent
  ]
})
export class SacrificalGoatModule { }
