import { TestBed } from '@angular/core/testing';

import { NavigationProviderService } from './navigation-provider.service';

describe('NavigationProviderService', () => {
  beforeEach(() => TestBed.configureTestingModule({}));

  it('should be created', () => {
    const service: NavigationProviderService = TestBed.get(NavigationProviderService);
    expect(service).toBeTruthy();
  });
});
