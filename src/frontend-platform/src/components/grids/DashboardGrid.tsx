import { DaoAssociation } from "@/declarations/dao/dao-association.types";

type DashboardGridProps = {
    daos: DaoAssociation[]
}

export const DashboardGrid = ({ daos }: DashboardGridProps) => {
    return (
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
            {daos.map((dao) => (
                <div key={dao.uid} className="bg-white rounded-lg shadow-md p-6">

                </div>
            ))}
        </div>
    );
};