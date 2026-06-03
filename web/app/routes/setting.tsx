import type { Route } from "./+types/setting";
import { Index } from "~/setting";

export function meta({}: Route.MetaArgs) {
  return [
    { title: "Setting" },
    { name: "description", content: "Welcome to Setting!" },
  ];
}

export default function Setting() {
  return <Index />;
}
