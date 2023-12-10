import { z } from 'zod';

import { SimpleStructure } from './simple-structure.zod';

export const Reference = z.object({
  reference: z.lazy(() => SimpleStructure),
});

export type Reference = z.infer<typeof Reference>;
