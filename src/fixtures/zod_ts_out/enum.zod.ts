import { z } from 'zod';

export const Enumeration = z.enum([
  'a',
  'is',
  'test',
  'this',
]);

export type Enumeration = z.infer<typeof Enumeration>;
