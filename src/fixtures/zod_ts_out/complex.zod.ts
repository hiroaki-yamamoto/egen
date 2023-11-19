import { z } from 'zod';

import { SimpleStructure } from './simple-structure.zod.ts';
import { Reference } from './reference.zod.ts';

export const Complex = z.object({
  code: z.number().max(65535).min(0),
  simpleText: z.string(),
  simpleArray: z.array(z.string()),
  referenceArray: z.array(z.lazy(() => SimpleStructure)),
  selfReferenceArray: z.array(z.lazy(() => Complex)),
  secondReference: z.lazy(() => Reference),
});
