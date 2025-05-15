import { OrganizationStatus } from "@/models/entities/Dao";

interface DaoStatusBadgeProps {
    status: OrganizationStatus;
}

export const DaoStatusBadge = ({ status }: DaoStatusBadgeProps) => {
    const getStatusColor = (status: OrganizationStatus) => {
        switch (status) {
            case OrganizationStatus.Active:
                return "text-green-600 bg-green-50";
            case OrganizationStatus.Liquidation:
                return "text-yellow-600 bg-yellow-50";
            case OrganizationStatus.Dissolved:
                return "text-red-600 bg-red-50";
            default:
                return "text-gray-600 bg-gray-50";
        }
    };

    return (
        <span className={`px-2 py-1 rounded-full text-xs font-medium ${getStatusColor(status)}`}>
            {status}
        </span>
    );
}