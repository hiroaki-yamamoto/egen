import { z } from 'zod';

import { Reference } from './reference.zod';
import { SimpleStructure } from './simple-structure.zod';

export const Complex = z.object({
  code: z.number().int().max(65535).min(0),
  detailedText: z.string(),
  referenceArray: z.array(z.lazy(() => SimpleStructure)),
  secondReference: z.lazy(() => Reference),
  selfReferenceArray: z.array(z.lazy(() => Complex)),
  simpleArray: z.array(z.string()),
  simpleText: z.string(),
});

export type Complex = z.infer<typeof Complex>;
