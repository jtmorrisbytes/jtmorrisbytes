import { async, ComponentFixture, TestBed } from '@angular/core/testing';

import { GreenLambComponent } from './green-lamb.component';

describe('GreenLambComponent', () => {
  let component: GreenLambComponent;
  let fixture: ComponentFixture<GreenLambComponent>;

  beforeEach(async(() => {
    TestBed.configureTestingModule({
      declarations: [ GreenLambComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(GreenLambComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
