import { z } from 'zod';

export const SelfReference = z.object({
  reference: z.lazy(() => SelfReference),
});
