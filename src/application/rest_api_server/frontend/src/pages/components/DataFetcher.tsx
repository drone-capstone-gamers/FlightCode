import React, { useState, useEffect } from 'react';

interface Props {
  dataSource: string
}

const DataFetcher = ({ dataSource }: Props) => {
  const [data, setData] = useState(null);

  useEffect(() => {
    const fetchData = async () => {
      try {
        const response = await fetch(`/api/${dataSource}`);
        const jsonData = await response.json();
        setData(jsonData);
      } catch (error) {
        console.error('Error fetching data:', error);
      }
    };

    fetchData();
  }, [dataSource]);

  return (
    <div className='border-r border-gray-500'>
      <h2 className='text-emerald-200'>{dataSource.toUpperCase()}</h2>
      {data ? (
        <div>
          <ul>
            {Object.entries(data).map(([key, value]) => (
              <li key={key}>
                <strong>{key}:</strong> {JSON.stringify(value)}
              </li>
            ))}
          </ul>
        </div>
      ) : (
        <p>Fetching {dataSource}...</p>
      )}
  </div>
);
};

export default DataFetcher;
