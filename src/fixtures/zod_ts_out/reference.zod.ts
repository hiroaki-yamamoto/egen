import { z } from 'zod';

import { SimpleStructure } from './simple-structure.zod.ts';

export const Reference = z.object({
  reference: z.lazy(() => SimpleStructure),
});

export type Reference = z.infer<typeof Reference>;
