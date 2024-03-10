import DataFetcher from "@/pages/components/DataFetcher";
import ImageFetcher from "@/pages/components/ImageFetcher";

export default function Home() {
  return (
    <main
      className={`flex min-h-screen flex-col items-center gap-5 p-10`}
    >
      <div className="inline-grid lg:grid-cols-3 gap-5 border border-gray-500">
        <DataFetcher dataSource={"example"} />
        <DataFetcher dataSource={"power"} />
        <DataFetcher dataSource={"temperature"} />
        <DataFetcher dataSource={"obc_telemetry"} />
        <DataFetcher dataSource={"global_position"} />
        <DataFetcher dataSource={"imu"} />
        <DataFetcher dataSource={"environmental"} />
      </div>

      <div className="inline-grid lg:grid-cols-3 gap-x-5 gap-y-20">
        <ImageFetcher dataSource={"thermal_img"} />
        <ImageFetcher dataSource={"picam_image"} />
      </div>
    </main>
  );
}
