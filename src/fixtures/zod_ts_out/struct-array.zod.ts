import { z } from 'zod';

export const StructArray = z.object({
  lst: z.array(z.string()),
});
