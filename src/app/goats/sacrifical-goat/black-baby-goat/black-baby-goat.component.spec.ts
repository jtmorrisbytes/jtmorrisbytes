import { async, ComponentFixture, TestBed } from '@angular/core/testing';

import { BlackBabyGoatComponent } from './black-baby-goat.component';

describe('BlackBabyGoatComponent', () => {
  let component: BlackBabyGoatComponent;
  let fixture: ComponentFixture<BlackBabyGoatComponent>;

  beforeEach(async(() => {
    TestBed.configureTestingModule({
      declarations: [ BlackBabyGoatComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(BlackBabyGoatComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
