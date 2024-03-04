import DataFetcher from "@/pages/components/DataFetcher";

export default function Home() {
  return (
    <main
      className={`flex min-h-screen flex-col items-center justify-between p-24`}
    >
      <div className="relative flex before:absolute">
        <div className="inline-grid grid-cols-3 gap-5 border border-gray-500">
          <div>
            <DataFetcher dataSource={"example"} />
          </div>
          <div>
            <DataFetcher dataSource={"power"} />
          </div>
        </div>
      </div>
    </main>
  );
}
