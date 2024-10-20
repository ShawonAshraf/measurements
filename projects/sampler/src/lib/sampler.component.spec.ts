import { ComponentFixture, TestBed } from '@angular/core/testing';

import { SamplerComponent } from './sampler.component';

describe('SamplerComponent', () => {
  let component: SamplerComponent;
  let fixture: ComponentFixture<SamplerComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [SamplerComponent]
    })
    .compileComponents();

    fixture = TestBed.createComponent(SamplerComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
