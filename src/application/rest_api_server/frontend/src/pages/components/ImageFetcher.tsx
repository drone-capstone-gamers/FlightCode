import React, { useState } from 'react';

interface Props {
    dataSource: string
}

const ImageFetcher = ({ dataSource }: Props) => {
    const [image, setImage] = useState('');

    const fetchImage = async () => {
        try {
            const response = await fetch(`/api/${dataSource}`);
            const blob = await response.blob();
            const imageSrc = URL.createObjectURL(blob);
            setImage(imageSrc);
        } catch (error) {
            console.error('Error fetching image:', error);
        }
    };

    return (
        <div className="w-72 h-40">
            <h2 className='text-emerald-200'>{dataSource ? dataSource.toUpperCase() : ''}</h2>
            <button onClick={fetchImage}
                    className="mt-2 p-2 bg-emerald-200 text-cyan-900 rounded-md hover:bg-emerald-600 focus:outline-none focus:ring focus:border-emerald-400">
                Update with latest
            </button>
            {image && <img className="w-full h-full object-cover" src={image} alt="Image unavailable" />}
        </div>
    );
};

export default ImageFetcher;