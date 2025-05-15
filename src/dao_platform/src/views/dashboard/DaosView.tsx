import { DaoCard } from "@/components/cards/DaoCard";
import { Dao } from "@/models/entities/Dao";

interface DaosViewProps {
    title: string;
    daos: Dao[];
} 

export const DaosView = ({ title, daos }: DaosViewProps) => {
    return (
        <div>
            <h2 className="text-2xl font-bold mb-6">{title}</h2>
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                {daos.map(dao => (
                    <DaoCard key={dao.uid} dao={dao} />
                ))}
            </div>
        </div>
    );
}