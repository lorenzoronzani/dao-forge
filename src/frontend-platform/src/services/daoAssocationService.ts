export const fetchDaoAssociations = async () => {
    const response = await fetch("/api/daos");
    if (!response.ok) throw new Error("Failed to fetch DAOs");
    return response.json();
};