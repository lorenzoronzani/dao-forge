import { Dao } from "@/models/entities/Dao";
import { DaoStatusBadge } from "@/components/badge/DaoStatusBadge";
import { Building, MapPin, Users, Calendar, FileText } from "lucide-react";
import { useState } from "react";
import { HorizontalLabeledField } from "@/components/labels/HorizontalLabeledField";
import { formatDate } from "@/utils/date";

interface DaoInformationProps {
    dao: Dao;
}

export const DaoInformation = ({ dao }: DaoInformationProps) => {
    const [showFullPurpose, setShowFullPurpose] = useState<boolean>(false);

    return (
        <>
            <div className="mb-6">
                <div className="flex justify-between items-center mb-2">
                    <h1 className="text-2xl font-bold">{dao.name}</h1>
                    <DaoStatusBadge status={dao.status} />
                </div>

                <div className="bg-white p-4 rounded-lg shadow-sm mb-3">
                    <div className="grid grid-cols-2 md:grid-cols-4 gap-x-6 gap-y-2">
                        <HorizontalLabeledField image={<Building className="h-4 w-4 text-slate-500" />} label="Legal Form:" text={dao.legalForm} />
                        <HorizontalLabeledField image={<MapPin className="h-4 w-4 text-slate-500" />} label="Location:" text={`${dao.address}, ${dao.town}, ${dao.zip}`} />
                        <HorizontalLabeledField image={<Users className="h-4 w-4 text-slate-500" />} label="People:" text={`${dao.board.length} board, ${dao.members.length} members`} />
                        <HorizontalLabeledField image={<Calendar className="h-4 w-4 text-slate-500" />} label="Created:" text={formatDate(dao.createdAt.getTime())} />
                    </div>
                </div>

                <div className="bg-white p-4 rounded-lg shadow-sm">
                    <div className="flex items-center justify-between mb-1">
                        <p className="text-slate-500 text-sm flex items-center">
                            <FileText className="h-4 w-4 mr-1" />
                            Purpose
                        </p>
                        {dao.purpose && dao.purpose.length > 100 && (
                            <button
                                onClick={() => setShowFullPurpose(!showFullPurpose)}
                                className="text-xs text-slate-500 flex items-center"
                            >
                                {showFullPurpose ? 'Show less' : 'Show more'}
                                <svg
                                    className={`h-4 w-4 ml-1 transform ${showFullPurpose ? 'rotate-180' : ''}`}
                                    fill="none"
                                    viewBox="0 0 24 24"
                                    stroke="currentColor"
                                >
                                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 9l-7 7-7-7" />
                                </svg>
                            </button>
                        )}
                    </div>
                    <div className={`text-sm ${showFullPurpose ? '' : 'line-clamp-2'}`}>
                        {dao.purpose}
                    </div>
                </div>
            </div>
        </>
    );
}