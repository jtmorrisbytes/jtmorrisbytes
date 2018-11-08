import { async, ComponentFixture, TestBed } from '@angular/core/testing';

import { RedLambComponent } from './red-lamb.component';

describe('RedLambComponent', () => {
  let component: RedLambComponent;
  let fixture: ComponentFixture<RedLambComponent>;

  beforeEach(async(() => {
    TestBed.configureTestingModule({
      declarations: [ RedLambComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(RedLambComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
