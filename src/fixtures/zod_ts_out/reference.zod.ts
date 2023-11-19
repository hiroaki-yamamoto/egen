import { z } from 'zod';

import { SimpleStructure } from './simple-structure.zod.ts';

export const Reference = z.object({
  reference: SimpleStructure,
});
