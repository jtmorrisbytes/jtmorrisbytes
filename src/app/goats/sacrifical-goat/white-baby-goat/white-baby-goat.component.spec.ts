import { async, ComponentFixture, TestBed } from '@angular/core/testing';

import { WhiteBabyGoatComponent } from './white-baby-goat.component';

describe('WhiteBabyGoatComponent', () => {
  let component: WhiteBabyGoatComponent;
  let fixture: ComponentFixture<WhiteBabyGoatComponent>;

  beforeEach(async(() => {
    TestBed.configureTestingModule({
      declarations: [ WhiteBabyGoatComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(WhiteBabyGoatComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
