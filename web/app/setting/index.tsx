import { useEffect } from "react";
import instance from "~/axios";
export function Index() {
  useEffect(() => {
    instance.get("/api/setting").then((response) => {
      console.log("Setting data:", response.data);
    }).catch((error) => {
      console.error("Error fetching setting data:", error);
    });
    console.log("Setting page loaded");
  }, []);


  return (
    <main className="flex flex-col gap-4 p-4">
      Setting some
      <div>
        <label>Storage Root:</label>
        <input placeholder="Enter setting..." />
      </div>

    </main>
  );
}
