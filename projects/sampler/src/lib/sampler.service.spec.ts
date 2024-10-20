import { TestBed } from '@angular/core/testing';

import { SamplerService } from './sampler.service';

describe('SamplerService', () => {
  let service: SamplerService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(SamplerService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
